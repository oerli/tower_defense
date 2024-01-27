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
    query_level: Query<Entity, With<Level>>,
    round: Res<RoundHandle>,
    mut rounds: ResMut<Assets<Round>>,
    mut current_round: ResMut<CurrentRound>,
) {
    if let Ok(_level_entity) = query_level.get_single() {
        if let Some(round) = rounds.remove(round.0.id()) {
            current_round.timer =
                Timer::from_seconds(round.separation_time.clone(), TimerMode::Repeating);
            current_round.round = Some(round);
        }
    }
}

pub fn spawn_enemies(
    time: Res<Time>,
    mut event_writer: EventWriter<SpawnEnemyEvent>,
    mut player: ResMut<Player>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut current_round: ResMut<CurrentRound>,
    query_enemy: Query<&Enemy>,
) {
    current_round.timer.tick(time.delta());

    if current_round.timer.finished() {
        if let Some(round) = current_round.round.as_mut() {

            // if round is finished remove it otherwise spawn an enemy
            if round.enemy_count <= 0 {
                // check if all enemies are destroyed
                if query_enemy.iter().count() == 0 {
                    // give player some credits, but not if it's the last round
                    player.credits += 10;

                    // set index to next round and remove current round
                    current_round.index += 1;
                    current_round.round = None;

                    // show menu to choose from tower for next round
                    next_game_state.set(GameState::Paused);
                }
            } else {
                round.enemy_count -= 1;
                event_writer.send(SpawnEnemyEvent {
                    enemy: round.enemy.clone(),
                    position: Vec3::new(-8.0, 0.0, -8.0),
                });
            }
        }
    };
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

pub fn load_rounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_round: Res<CurrentRound>,
    query_level: Query<&Level>,
) {
    if current_round.round.is_none()
        && current_round.index < query_level.get_single().unwrap().rounds
    {
        let round = RoundHandle(
            asset_server.load(format!("levels/{:02}.round.toml", current_round.index + 1)),
        );
        commands.insert_resource(round);
    }
}
