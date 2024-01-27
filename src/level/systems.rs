use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use rand::Rng;

use crate::components::*;
use crate::enemy::components::*;
use crate::enemy::events::*;
use crate::player::resources::*;
use crate::resources::*;
use crate::GameState;

use super::components::*;
use super::resources::*;

pub fn setup_level(
    mut commands: Commands,
    query_tiles: Query<(Entity, &Transform), With<Tile>>,
    query_children: Query<&Children>,
    asset_server: Res<AssetServer>,
    level: Res<LevelHandle>,
    mut levels: ResMut<Assets<Level>>,
) {
    if let Some(level) = levels.remove(level.0.id()) {
        let mut rng = rand::thread_rng();

        for (entity, transform) in query_tiles.iter() {
            for position in level.waypoints.iter() {
                if transform.translation.x == position.x && transform.translation.z == position.z {
                    commands.entity(entity).remove::<On<Pointer<Click>>>();
                    commands.entity(entity).remove::<On<Pointer<Over>>>();
                    commands.entity(entity).remove::<SceneBundle>();

                    // remove possible decorations
                    match query_children.get(entity) {
                        Ok(children) => {
                            for child in children.iter() {
                                commands.entity(*child).remove::<SceneBundle>();
                            }
                        }
                        Err(_) => {}
                    }

                    commands
                        .entity(entity)
                        .insert(SceneBundle {
                            scene: asset_server.load("models/path.glb#Scene0"),
                            transform: Transform::from_xyz(position.x, 0.0, position.z),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // create some dirt on street
                            if 0.3 > rng.gen() {
                                parent.spawn(SceneBundle {
                                    scene: asset_server.load("models/dirt.glb#Scene0"),
                                    transform: Transform::from_xyz(0.0, 0.2, 0.0),
                                    ..Default::default()
                                });
                            }
                        });
                }
            }
        }

        // spawn level
        commands.spawn(level);
    }
}

pub fn setup_round(
    mut commands: Commands,
    query_level: Query<Entity, With<Level>>,
    round: Res<RoundHandle>,
    mut rounds: ResMut<Assets<Round>>,
) {
    if let Ok(level_entity) = query_level.get_single() {
        if let Some(round) = rounds.remove(round.0.id()) {
            commands.entity(level_entity).with_children(|parent| {
                parent.spawn((
                    Lifetime {
                        timer: Timer::from_seconds(
                            round.separation_time.clone(),
                            TimerMode::Repeating,
                        ),
                    },
                    round,
                ));
            });
        }
    }
}

pub fn spawn_enemies(
    // mut query_level: Query<&mut Level>,
    mut query: Query<(Entity, &mut Lifetime, &mut Round)>,
    time: Res<Time>,
    mut event_writer: EventWriter<SpawnEnemyEvent>,
    mut commands: Commands,
    mut player: ResMut<Player>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    // sort rounds by index
    let mut round_entities = query.iter_mut().collect::<Vec<_>>();

    round_entities.sort_by_key(|(_, _, round)| round.index);

    let round_entries = round_entities.len();

    for (entity, mut lifetime, mut round) in round_entities {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.finished() {
            // if round is finished remove it otherwise spawn an enemy
            if round.enemy_count <= 0 {
                // give player some credits, but not if it's the last round
                if round_entries > 1 {
                    player.credits += 10;

                    // show menu to choose from tower for next round
                    next_game_state.set(GameState::Paused);
                }

                commands.entity(entity).despawn();
            } else {
                round.enemy_count -= 1;
                event_writer.send(SpawnEnemyEvent {
                    enemy: round.enemy.clone(),
                    position: Vec3::new(-8.0, 0.0, -8.0),
                });
            }
        }

        // single query only for first in list
        break;
    }
}

pub fn load_levels(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = LevelHandle(asset_server.load("levels/first.level.toml"));
    commands.insert_resource(level);

    // load animations for orc
    commands.insert_resource(Animations(vec![
        // running animation
        asset_server.load("models/orc.glb#Animation3"),
        // dying animation
        asset_server.load("models/orc.glb#Animation9"),
        // jumping animation
        asset_server.load("models/orc.glb#Animation5"),
    ]));
}

pub fn load_rounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let round = RoundHandle(asset_server.load("levels/first.round.toml"));
    commands.insert_resource(round);
    // let round = RoundHandle(asset_server.load("levels/second.round.toml"));
    // commands.insert_resource(round);
    // let round = RoundHandle(asset_server.load("levels/third.round.toml"));
    // commands.insert_resource(round);
}
