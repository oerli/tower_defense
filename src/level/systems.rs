use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::info;
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
        let heights = vec![
            [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
         ];
        // for (x, z_line) in heights.iter().enumerate() {
        //     for (z , height) in z_line.iter().enumerate() {
        //         info!("{:?}", tile_type(&heights, x, z));

        //     }
        // }

        // create tiles
        let mut waypoints = Vec::new();
        for (x, z_line) in level.map.iter().enumerate() {
            for (z, path_index) in z_line.iter().enumerate() {
                // select specific tile from sorrounding heights from the heights map

                // create empty tiles
                if *path_index == 0 {
                    commands
                        .spawn((
                            // SceneBundle {
                            //     scene: asset_server.load(tile_type(x, z, false)),
                            //     transform: Transform::from_xyz(
                            //         x as f32,
                            //         heights[x][z] * 0.2,
                            //         z as f32,
                            //     ),
                            //     ..Default::default()
                            // },
                            TransformBundle::from_transform(Transform::from_xyz(
                                x as f32,
                                heights[x][z] * 0.2 - 0.1,
                                z as f32,
                            )),
                            VisibilityBundle::default(),
                            PickableBundle::default(),
                            RapierPickable,
                            Collider::cuboid(0.50, 0.1, 0.5),
                            CollisionGroups::new(Group::GROUP_5, Group::GROUP_5),
                            On::<Pointer<Click>>::send_event::<BuildEvent>(),
                            On::<Pointer<Over>>::send_event::<OverEvent>(),
                            // On::<Pointer<Out>>::send_event::<OutEvent>(),
                            Tile,
                        ))
                        .with_children(|parent| {
                            parent.spawn((SceneBundle {
                                scene: asset_server.load(tile_type(x, z, false)),
                                transform: Transform::from_xyz(0.0, -0.1, 0.0).with_rotation(tile_rotaton(x, z)),
                                ..Default::default()
                            },));
                            // create some trees or rocks
                            if 0.2 > rng.gen() {
                                parent.spawn(SceneBundle {
                                    scene: asset_server.load("models/tree.glb#Scene0"),
                                    transform: Transform::from_xyz(0.0, 0.1, 0.0)
                                        .with_scale(Vec3::splat(2.0)),
                                    ..Default::default()
                                });
                            } else if 0.1 > rng.gen() {
                                parent.spawn(SceneBundle {
                                    scene: asset_server.load("models/rocks.glb#Scene0"),
                                    transform: Transform::from_xyz(0.0, 0.1, 0.0),
                                    ..Default::default()
                                });
                            }
                        });
                } else {
                    // collect all waypoints
                    waypoints.push((
                        path_index,
                        // 0.2 size of tiles + 0.5 of character offset
                        Vec3::new(x as f32, heights[x][z] * 0.2 + 0.5, z as f32),
                    ));
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
                        TransformBundle::from_transform(
                            Transform::from_xyz(
                                position.x,
                                heights[position.x as usize][position.z as usize] * 0.2 - 0.1,
                                position.z,
                            )
                            .with_rotation(rotation),
                        ),
                        VisibilityBundle::default(),
                        // SceneBundle {
                        //     scene: asset_server.load("models/tile_end.glb#Scene0"),
                        //     transform: Transform::from_xyz(
                        //         position.x,
                        //         heights[position.x as usize][position.z as usize] * 0.2-0.2,
                        //         position.z,
                        //     )
                        //     .with_rotation(rotation),
                        //     ..Default::default()
                        // },
                        Tile,
                    ))
                    .with_children(|parent| {
                        parent.spawn(SceneBundle {
                            scene: asset_server.load("models/tile_end.glb#Scene0"),
                            transform: Transform::from_xyz(0.0, -0.1, 0.0),
                            ..Default::default()
                        });
                        parent.spawn(SceneBundle {
                            scene: asset_server.load("models/arc.glb#Scene0"),
                            transform: Transform::from_xyz(0.0, 0.1, 0.0),
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
                        TransformBundle::from_transform(
                            Transform::from_xyz(
                                position.x,
                                heights[position.x as usize][position.z as usize] * 0.2 - 0.1,
                                position.z,
                            )
                            .with_rotation(rotation),
                        ),
                        VisibilityBundle::default(),
                        Tile,
                    ))
                    .with_children(|parent| {
                        parent.spawn(SceneBundle {
                            scene: asset_server.load("models/tile_end.glb#Scene0"),
                            transform: Transform::from_xyz(0.0, -0.1, 0.0),
                            ..Default::default()
                        });
                        parent.spawn(SceneBundle {
                            scene: asset_server.load("models/arc.glb#Scene0"),
                            transform: Transform::from_xyz(0.0, 0.1, -0.4),
                            ..Default::default()
                        });
                        parent.spawn(SceneBundle {
                            scene: asset_server.load("models/banner.glb#Scene0"),
                            transform: Transform::from_xyz(0.0, 0.1, 0.0),
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
                    let rotation = if tile_rotaton(position.x as usize, position.z as usize) != Quat::default() {
                        tile_rotaton(position.x as usize, position.z as usize)
                    } else {
                        Quat::from_rotation_y(forward_rotation_angle)
                    };
                    // let rotation = Quat::from_rotation_y(forward_rotation_angle);
                    commands.spawn((
                        TransformBundle::from_transform(
                            Transform::from_xyz(
                                position.x,
                                heights[position.x as usize][position.z as usize] * 0.2 - 0.1,
                                position.z,
                            ).with_rotation(rotation),
                        ),
                        VisibilityBundle::default(),
                        Tile,
                    )).with_children(|parent| {
                        parent.spawn(SceneBundle {
                            scene: asset_server.load(tile_type(
                                position.x as usize,
                                position.z as usize,
                                true,
                            )),
                            transform: Transform::from_xyz(0.0, -0.1, 0.0),
                            ..Default::default()
                        });
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

                    commands
                        .spawn((
                            // SceneBundle {
                            //     scene: asset_server.load("models/tile_corner.glb#Scene0"),
                            //     transform: Transform::from_xyz(
                            //         position.x,
                            //         heights[position.x as usize][position.z as usize] * 0.2,
                            //         position.z,
                            //     )
                            //     .with_rotation(rotation),
                            //     ..Default::default()
                            // },
                            TransformBundle::from_transform(Transform::from_xyz(
                                position.x,
                                heights[position.x as usize][position.z as usize] * 0.2 - 0.1,
                                position.z,
                            ).with_rotation(rotation)),
                            VisibilityBundle::default(),
                            Tile,
                        ))
                        .with_children(|parent| {
                            parent.spawn(SceneBundle {
                                scene: asset_server.load("models/tile_corner.glb#Scene0"),
                                transform: Transform::from_xyz(0.0, -0.1, 0.0),
                                ..Default::default()
                            });
                        });
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

fn tile_type(x: usize, z: usize, path: bool) -> String {
    let heights = vec![
            [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
         ];
        // check if the current element is on a corner
    if (z == 0 || z == heights.len() - 1) && (x == 0 || x == heights[z].len() - 1) {
        return "models/outer_corner.glb#Scene0".to_string();

        // return TileType::Corner;
    }

    // check if the current element is on an edge
    if x == 0 || z == heights.len() - 1 || z == 0 || x == heights[z].len() - 1 {
        return "models/slope.glb#Scene0".to_string();
        // return TileType::Edge;
    }


    // slopes on the z axis
    if  heights[z][x] < heights[z + 1][x] || heights[z][x] < heights[z - 1][x] ||
        // slopes on the x axis
        heights[z][x] < heights[z][x + 1] || heights[z][x] < heights[z][x - 1]
        {
        // return TileType::Edge;
        if path {
            return "models/tile_slope.glb#Scene0".to_string();
        } else {
            return "models/slope.glb#Scene0".to_string();
        }
    }

    // corner in field
    if (heights[z][x] < heights[z+1][x+1]) || (heights[z][x] < heights[z-1][x+1]) || (heights[z][x] < heights[z+1][x-1] || heights[z][x] < heights[z-1][x-1]) {
        return "models/outer_corner.glb#Scene0".to_string();
    }

    // TileType::Flat
    if path {
        return "models/tile_straight.glb#Scene0".to_string();
    } else {
        return "models/tile.glb#Scene0".to_string();
    }
}

// todo: optimize this
fn tile_rotaton(x: usize, z: usize) -> Quat {
    let heights = vec![
            [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0],
            [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
         ];
    // check if the current element is a corner on the edge of the field
    if z == 0  && x == heights[z].len() - 1 {
        return Quat::from_rotation_y(0.0);
    }

    if x == 0  && z == 0  {
        return Quat::from_rotation_y( PI / 2.0);
    }
    if z == heights.len() - 1 && x == 0  {
        return Quat::from_rotation_y(PI);
    }

    if z == heights.len() - 1 && x == heights[z].len() - 1 {
        return Quat::from_rotation_y(-PI / 2.0);
    }


    // outer side
    if x == 0  {
        return Quat::from_rotation_y(PI /2.0);
    }

    if z == 0 {
        return Quat::from_rotation_y(0.0);
    }

    if x == heights[z].len() - 1 {
        return Quat::from_rotation_y( -PI / 2.0);
    }

    if z == heights.len() - 1  {
        return Quat::from_rotation_y(PI);
    }

    // inner slopes
    if heights[z][x] < heights[z+1][x] {
        return Quat::from_rotation_y(0.0);
    }

    if heights[z][x] < heights[z-1][x] {
        
        return Quat::from_rotation_y(PI);
    }

    if heights[z][x] < heights[z][x + 1] {
        return Quat::from_rotation_y(PI/2.0);
    }

    if heights[z][x] < heights[z][x - 1] {
        return Quat::from_rotation_y(-PI/2.0);
    }

    // corners in field
    if heights[z][x] < heights[z+1][x+1] {
        return Quat::from_rotation_y( PI / 2.0);
    }
    if heights[z][x] < heights[z-1][x+1] {
        return Quat::from_rotation_y(PI);
    }
    if heights[z][x] < heights[z+1][x-1] {
        return Quat::from_rotation_y(0.0);
    }
    if heights[z][x] < heights[z-1][x-1] {
        return Quat::from_rotation_y(-PI / 2.0);
    }

    return Quat::from_rotation_y(0.0);
}