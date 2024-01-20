use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{bullet::components::Bullet, defense::components::Defense, enemy::components::Enemy};

use super::components::*;

// TODO: check if there is a better way to check types
pub fn enemy_contact(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut defense_query: Query<&mut Defense>,
    mut enemy_query: Query<(&mut Enemy, &Children)>,
    health_query: Query<&EnemyHealth>,
    bullet_query: Query<&Bullet>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                // bullet hit enemy
                bullet_query.get(*entity1).ok().map(|bullet| {
                    enemy_query
                        .get_mut(*entity2)
                        .ok()
                        .map(|(mut enemy, children)| {
                            enemy.health -= bullet.damage;
                            for child in children.iter() {
                                // despawn one health bar indicator
                                if let Ok(_health) = health_query.get(*child) {
                                    commands.entity(*child).despawn_recursive();
                                    break;
                                }
                            }
                        });

                    commands.entity(*entity1).despawn_recursive();
                });

                // enemy is in range
                enemy_query.get_mut(*entity2).ok().map(|_enemy| {
                    defense_query.get_mut(*entity1).ok().map(|mut defense| {
                        defense.targets.push_back(*entity2);
                    });
                });

                // bullet hit enemy
                bullet_query.get(*entity2).ok().map(|bullet| {
                    enemy_query
                        .get_mut(*entity1)
                        .ok()
                        .map(|(mut enemy, children)| {
                            enemy.health -= bullet.damage;
                            for child in children.iter() {
                                // despawn one health bar indicator
                                if let Ok(_health) = health_query.get(*child) {
                                    commands.entity(*child).despawn_recursive();
                                    break;
                                }
                            }
                        });

                    commands.entity(*entity2).despawn_recursive();
                });

                // enemy is in range
                enemy_query.get_mut(*entity1).ok().map(|_enemy| {
                    defense_query.get_mut(*entity2).ok().map(|mut defense| {
                        defense.targets.push_back(*entity1);
                    });
                });
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                enemy_query.get(*entity2).ok().map(|_enemy| {
                    defense_query.get_mut(*entity1).ok().map(|mut defense| {
                        defense.targets.retain(|&x| x != *entity2);
                    });
                });

                enemy_query.get(*entity1).ok().map(|_enemy| {
                    defense_query.get_mut(*entity2).ok().map(|mut defense| {
                        defense.targets.retain(|&x| x != *entity1);
                    });
                });
            }
        }
    }
}

#[derive(Event)]
pub struct SpawnEnemyEvent {
    pub enemy: Enemy,
    pub position: Vec3,
}

pub fn spawn_enemy (mut enemy_events: EventReader<SpawnEnemyEvent>, mut commands: Commands, asset_server: Res<AssetServer>) {
    for event in enemy_events.read() {
    commands
        .spawn((
            SceneBundle {
                scene: asset_server.load("models/orc.glb#Scene0"),
                transform: Transform::from_translation(event.position),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
            Velocity {
                linvel: Vec3::new(0.0, 0.0, 0.0),
                angvel: Vec3::new(0.0, 0.0, 0.0),
            },
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(
                Group::GROUP_3,
                Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_4,
            ),
            event.enemy.clone(),
        ))
        .with_children(|parent| {
            for health in 1..event.enemy.health + 1 {
                if health % 2 == 0 {
                    parent.spawn((
                        SceneBundle {
                            scene: asset_server.load("models/health.glb#Scene0"),
                            transform: Transform::from_xyz(health as f32 * 0.1, 1.0, 0.0),
                            ..Default::default()
                        },
                        EnemyHealth,
                    ));
                } else {
                    parent.spawn((
                        SceneBundle {
                            scene: asset_server.load("models/health.glb#Scene0"),
                            transform: Transform::from_xyz(
                                health as f32 * -0.1 + 0.1,
                                1.0,
                                0.0,
                            ),
                            ..Default::default()
                        },
                        EnemyHealth,
                    ));
                }
            }
        });
    }
}