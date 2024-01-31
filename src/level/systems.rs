use std::f32::consts::PI;

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
    if let Some(mut level) = levels.remove(level.0.id()) {
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
        let mut waypoints = Vec::new();
        for (x, z_line) in level.map.iter().enumerate() {
            for (z, path_index) in z_line.iter().enumerate() {
                // create empty tiles
                if *path_index == 0 {
                    commands
                        .spawn((
                            SceneBundle {
                                scene: asset_server.load("models/tile.glb#Scene0"),
                                transform: Transform::from_xyz(x as f32, 0.0, z as f32),
                                ..Default::default()
                            },
                            PickableBundle::default(),
                            RapierPickable,
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
                } else {
                    // collect all waypoints
                    waypoints.push((path_index, Vec3::new(x as f32, 0.0, z as f32)));
                }
            }
        }

        // sort waypoints by index (first element of tuple)
        waypoints.sort_by(|a, b| a.0.cmp(&b.0));

        // create path from waypoints, index must start with 1
        for (index, (_, position)) in waypoints.iter().enumerate() {
            // check if it's first tile
            if index == 0 {
                let (_, next_position) = waypoints.get(index + 1).unwrap();

                let direction = (*next_position - *position).normalize();
                let rotation_angle = direction.x.atan2(direction.z) + PI;
                let rotation = Quat::from_rotation_y(rotation_angle);

                commands
                    .spawn((
                        SceneBundle {
                            scene: asset_server.load("models/tile_end.glb#Scene0"),
                            transform: Transform::from_xyz(position.x, 0.0, position.z)
                                .with_rotation(rotation),
                            ..Default::default()
                        },
                        Tile,
                    ))
                    .with_children(|parent| {
                        parent.spawn(SceneBundle {
                            scene: asset_server.load("models/arc.glb#Scene0"),
                            transform: Transform::from_xyz(0.0, 0.2, 0.0),
                            ..Default::default()
                        });
                    });
            // check if it's last tile
            } else if index == waypoints.len() - 1 {
                let (_, previous_position) = waypoints.get(index - 1).unwrap();

                let direction = (*previous_position - *position).normalize();
                let rotation_angle = direction.x.atan2(direction.z) + PI;
                let rotation = Quat::from_rotation_y(rotation_angle);

                commands
                    .spawn((
                        SceneBundle {
                            scene: asset_server.load("models/tile_end.glb#Scene0"),
                            transform: Transform::from_xyz(position.x, 0.0, position.z)
                                .with_rotation(rotation),
                            ..Default::default()
                        },
                        Tile,
                    ))
                    .with_children(|parent| {
                        parent.spawn(SceneBundle {
                            scene: asset_server.load("models/arc.glb#Scene0"),
                            transform: Transform::from_xyz(0.0, 0.2, -0.6),
                            ..Default::default()
                        });
                        parent.spawn(SceneBundle {
                            scene: asset_server.load("models/banner.glb#Scene0"),
                            transform: Transform::from_xyz(0.0, 0.2, -0.2),
                            ..Default::default()
                        });
                    });
            } else {
                // if not first or last tile, indicies must exist
                let (_, previous_position) = waypoints.get(index - 1).unwrap().clone();
                let (_, next_position) = waypoints.get(index + 1).unwrap().clone();

                let backward_direction = (previous_position - *position).normalize();
                let forward_direction = (next_position - *position).normalize();

                let backward_rotation_angle = backward_direction.x.atan2(backward_direction.z) + PI;
                let forward_rotation_angle = forward_direction.x.atan2(forward_direction.z) + PI;

                // check if it's a straight tile
                if backward_direction.x == forward_direction.x
                    || backward_direction.z == forward_direction.z
                {
                    let rotation = Quat::from_rotation_y(forward_rotation_angle);
                    commands
                        .spawn((
                            SceneBundle {
                                scene: asset_server.load("models/tile_straight.glb#Scene0"),
                                transform: Transform::from_xyz(position.x, 0.0, position.z)
                                    .with_rotation(rotation),
                                ..Default::default()
                            },
                            Tile,
                        ))
                        .with_children(|parent| {
                            // create some dirt on street
                            if 0.3 > rng.gen() {
                                parent.spawn(SceneBundle {
                                    scene: asset_server.load("models/dirt.glb#Scene0"),
                                    transform: Transform::from_xyz(0.0, 0.1, 0.0),
                                    ..Default::default()
                                });
                            }
                        });
                } else {
                    // calculate the rotation of a corner tile from last, current and next tile
                    // todo: review if there would be any better option
                    let rotation = if forward_direction.x - backward_direction.x < 0.0
                        && forward_direction.z - backward_direction.z > 0.0
                    {
                        let rotation_angle = forward_rotation_angle - backward_rotation_angle;
                        Quat::from_rotation_y(rotation_angle - PI / 2.0)
                    } else if forward_direction.x - backward_direction.x < 0.0
                        && forward_direction.z - backward_direction.z < 0.0
                    {
                        let rotation_angle = forward_rotation_angle - backward_rotation_angle;
                        Quat::from_rotation_y(rotation_angle + PI)
                    } else if forward_direction.x - backward_direction.x > 0.0
                        && forward_direction.z - backward_direction.z > 0.0
                    {
                        let rotation_angle = forward_rotation_angle - backward_rotation_angle;
                        Quat::from_rotation_y(rotation_angle)
                    } else {
                        let rotation_angle = forward_rotation_angle - backward_rotation_angle;
                        Quat::from_rotation_y(rotation_angle + PI / 2.0)
                    };

                    commands.spawn((
                        SceneBundle {
                            scene: asset_server.load("models/tile_corner.glb#Scene0"),
                            transform: Transform::from_xyz(position.x, 0.0, position.z)
                                .with_rotation(rotation),
                            ..Default::default()
                        },
                        Tile,
                    ));
                }
            }
        }


        // collect all positions from waypoints
        let mut waypoints_level = Vec::new();
        for (_, position) in waypoints.iter() {
            waypoints_level.push(position.clone());
        }

        // populate waypoints into level
        level.waypoints = Some(waypoints_level);

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
                        player.credits += 1;

                        // set index to next round and remove current round
                        current_round.index += 1;
                        current_round.round = None;

                        // show menu to choose from tower for next round
                        next_game_state.set(GameState::RoundEnded);
                    }
                } else {
                    if let Some(waypoints) = &level.waypoints {
                        round.enemy_count -= 1;
                        event_writer.send(SpawnEnemyEvent {
                            enemy: round.enemy.clone(),
                            position: waypoints.get(0).unwrap().clone(),
                        });
                    }
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
            let round_handle = RoundHandle(asset_server.load(format!(
                "levels/{:02}.{:02}.round.toml",
                current_level.index + 1,
                current_round.index + 1
            )));
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
