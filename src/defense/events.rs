use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;
use super::resources::*;
use crate::player::resources::*;
use crate::BuildEvent;

pub fn spawn_defense(
    mut build_events: EventReader<BuildEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    defense_selection: Res<DefenseSelection>,
    mut player: ResMut<Player>,
    transform_query: Query<&GlobalTransform>,
) {
    for event in build_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }

        // check if the player has enough money
        if player.score < 10 {
            continue;
        } else {
            player.score -= 10;
        }

        match defense_selection.selected {
            Weapon::Cannon => {
                // spawn the defense
                transform_query.get(event.entity).ok().map(|transform| {
                    commands
                        .spawn((
                            SceneBundle {
                                scene: asset_server.load("models/cannon_tower.glb#Scene0"),
                                transform: transform.clone().into(),
                                ..Default::default()
                            },
                            RigidBody::Dynamic,
                            Defense {
                                targets: VecDeque::new(),
                                damage: 0.5,
                                shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                            },
                            Collider::ball(3.0),
                            Sensor,
                            CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
                            Pickable::IGNORE,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Collider::cuboid(0.5, 0.5, 0.5),
                                CollisionGroups::new(Group::GROUP_2, Group::GROUP_4),
                            ));
                            parent.spawn((
                                SceneBundle {
                                    scene: asset_server.load("models/cannon.glb#Scene0"),
                                    transform: Transform::from_xyz(0.0, 0.68, 0.0),
                                    ..Default::default()
                                },
                                Weapon::Cannon,
                            ));
                        });
                });
            }
            Weapon::Ballista => {
                // spawn the defense
                transform_query.get(event.entity).ok().map(|transform| {
                    commands
                        .spawn((
                            SceneBundle {
                                scene: asset_server.load("models/ballista_tower.glb#Scene0"),
                                transform: transform.clone().into(),
                                ..Default::default()
                            },
                            RigidBody::Dynamic,
                            Defense {
                                targets: VecDeque::new(),
                                damage: 0.3,
                                shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                            },
                            Collider::ball(3.0),
                            Sensor,
                            CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
                            Pickable::IGNORE,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Collider::cuboid(0.5, 0.5, 0.5),
                                CollisionGroups::new(Group::GROUP_2, Group::GROUP_4),
                            ));
                            parent.spawn((
                                SceneBundle {
                                    scene: asset_server.load("models/ballista.glb#Scene0"),
                                    transform: Transform::from_xyz(0.0, 0.68, 0.0),
                                    ..Default::default()
                                },
                                Weapon::Ballista,
                            ));
                        });
                });
            }
            Weapon::Archer => {
                // spawn the defense
                transform_query.get(event.entity).ok().map(|transform| {
                    commands
                        .spawn((
                            SceneBundle {
                                scene: asset_server.load("models/archer_tower.glb#Scene0"),
                                transform: transform.clone().into(),
                                ..Default::default()
                            },
                            RigidBody::Dynamic,
                            Defense {
                                targets: VecDeque::new(),
                                damage: 0.1,
                                shooting_timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                            },
                            Collider::ball(1.5),
                            Sensor,
                            CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
                            Pickable::IGNORE,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Collider::cuboid(0.5, 0.5, 0.5),
                                CollisionGroups::new(Group::GROUP_2, Group::GROUP_4),
                            ));
                            parent.spawn((Weapon::Archer,));
                        });
                });
            }
        }

        // disable the build event for multiple clicks
        commands.entity(event.entity).remove::<On<Pointer<Click>>>();
    }
}
