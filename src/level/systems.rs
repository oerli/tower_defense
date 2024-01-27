use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use rand::Rng;

use crate::components::*;
use crate::defense::components::*;
use crate::enemy::components::*;
use crate::enemy::events::*;
use crate::player::resources::*;
use crate::resources::*;
use crate::BuildEvent;
use crate::GameState;
use crate::OverEvent;

use super::components::*;
use super::resources::*;

pub fn setup_level(
    mut commands: Commands,
    query_tiles: Query<Entity, With<Tile>>,
    query_defense: Query<Entity, With<Defense>>,
    asset_server: Res<AssetServer>,
    level: Res<LevelHandle>,
    mut levels: ResMut<Assets<Level>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    if let Some(level) = levels.remove(level.0.id()) {
        let mut rng = rand::thread_rng();

        // remove all tiles
        for tile_entity in query_tiles.iter() {
            commands.entity(tile_entity).despawn_recursive();
        }
        // remvoe all defenses
        for defense_entity in query_defense.iter() {
            commands.entity(defense_entity).despawn_recursive();
        }

        // create tiles
        for x in -8..8 {
            for z in -8..8 {
                let mut tile_is_path = false;
                for position in level.waypoints.iter() {
                    // check if tile is a path
                    if x as f32 == position.x && z as f32 == position.z {
                        commands
                            .spawn((
                                SceneBundle {
                                    scene: asset_server.load("models/path.glb#Scene0"),
                                    transform: Transform::from_xyz(position.x, 0.0, position.z),
                                    ..Default::default()
                                },
                                Tile,
                            ))
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
                        tile_is_path = true;
                        break;
                    }
                }

                // create a empty tile
                if !tile_is_path {
                    commands
                        .spawn((
                            SceneBundle {
                                scene: asset_server.load("models/tile.glb#Scene0"),
                                transform: Transform::from_xyz(x as f32, 0.0, z as f32),
                                ..Default::default()
                            },
                            PickableBundle::default(),
                            Collider::cuboid(0.50, 0.2, 0.5),
                            CollisionGroups::new(Group::GROUP_5, Group::GROUP_5),
                            On::<Pointer<Click>>::send_event::<BuildEvent>(),
                            On::<Pointer<Over>>::send_event::<OverEvent>(),
                            // On::<Pointer<Out>>::send_event::<OutEvent>(),
                            Tile,
                        ))
                        .with_children(|parent| {
                            // create some trees or rocks
                            if 0.2 > rng.gen() {
                                parent.spawn(SceneBundle {
                                    scene: asset_server.load("models/tree.glb#Scene0"),
                                    transform: Transform::from_xyz(0.0, 0.2, 0.0)
                                        .with_scale(Vec3::splat(2.0)),
                                    ..Default::default()
                                });
                            } else if 0.1 > rng.gen() {
                                parent.spawn(SceneBundle {
                                    scene: asset_server.load("models/rocks.glb#Scene0"),
                                    transform: Transform::from_xyz(0.0, 0.2, 0.0),
                                    ..Default::default()
                                });
                            }
                        });
                }
            }
        }

        // set current level
        current_level.level = Some(level);
    }
}

pub fn setup_round(
    round: Res<RoundHandle>,
    mut rounds: ResMut<Assets<Round>>,
    mut current_round: ResMut<CurrentRound>,
) {
    if let Some(round) = rounds.remove(round.0.id()) {
        current_round.timer =
            Timer::from_seconds(round.separation_time.clone(), TimerMode::Repeating);
        current_round.round = Some(round);
    }
}

pub fn spawn_enemies(
    time: Res<Time>,
    mut event_writer: EventWriter<SpawnEnemyEvent>,
    mut player: ResMut<Player>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut current_round: ResMut<CurrentRound>,
    query_enemy: Query<&Enemy>,
    current_level: Res<CurrentLevel>,
) {
    if let Some(level) = &current_level.level {
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
                        next_game_state.set(GameState::RoundEnded);
                    }
                } else {
                    round.enemy_count -= 1;
                    event_writer.send(SpawnEnemyEvent {
                        enemy: round.enemy.clone(),
                        position: Vec3::new(
                            level.waypoints.get(0).unwrap().x,
                            0.0,
                            level.waypoints.get(0).unwrap().z,
                        ),
                    });
                }
            }
        };
    };
}

pub fn load_levels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
) {
    if current_level.level.is_none() {
        let level_hendle = LevelHandle(
            asset_server.load(format!("levels/{:02}.level.toml", current_level.index + 1)),
        );
        commands.insert_resource(level_hendle);
    }
}

pub fn load_rounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_round: ResMut<CurrentRound>,
    mut current_level: ResMut<CurrentLevel>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Some(level) = &current_level.level {
        if current_round.round.is_none() && current_round.index < level.rounds {
            let round_handle = RoundHandle(
                asset_server.load(format!("levels/{:02}.{:02}.round.toml", current_level.index + 1, current_round.index + 1)),
            );
            commands.insert_resource(round_handle);
        }

        if current_round.index >= level.rounds {
            // check if all levels are finished
            if level.last_level {
                next_game_state.set(GameState::GameOver);
            } else {
                // all rounds are finished remove current level
                current_level.level = None;
                // reset round
                current_round.index = 0;
                // set next level
                current_level.index += 1;
            }
        }
    }
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
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
