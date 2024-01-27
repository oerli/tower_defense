use std::f32::consts::PI;
use std::time::Duration;

use bevy::animation::RepeatAnimation;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::level::resources::*;
use crate::player::resources::*;
use crate::resources::*;

use super::components::*;

// TODO: review movement system
pub fn enemy_movement(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Enemy,
        &mut Velocity,
        &GlobalTransform,
        &mut Transform,
    )>,
    current_level: Res<CurrentLevel>,
    mut player: ResMut<Player>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    children: Query<&Children>,
) {
    if let Some(level) = &current_level.level {
        for (entity, mut enemy, mut velocity, position, mut transform) in query.iter_mut() {
            if enemy.waypoint < level.waypoints.len() && enemy.health > 0.0 {
                transform.look_at(level.waypoints[enemy.waypoint], Vec3::ZERO);
                // TODO: dirty hack to rotate the enemy 180 degrees
                transform.rotate(Quat::from_rotation_y(PI));

                let mut direction = level.waypoints[enemy.waypoint] - position.translation();
                direction.y = 0.0;
                let distance = direction.length();

                if distance < 0.5 {
                    enemy.waypoint += 1;
                } else {
                    direction = direction.normalize();
                    velocity.linvel += direction * enemy.speed;
                }
            } else if enemy.health > 0.0 {
                // enemy reached goal
                player.lives -= 1;

                for entity in children.iter_descendants(entity) {
                    if let Ok(mut animation_player) = animation_players.get_mut(entity) {
                        animation_player
                            .play_with_transition(
                                animations.0[2].clone_weak(),
                                Duration::from_millis(250),
                            )
                            .set_repeat(RepeatAnimation::Count(8));
                    }
                }
                commands.entity(entity).remove::<Enemy>();
                // despawn the enemy after 3 seconds
                commands.entity(entity).insert(DespawnTimer {
                    timer: Timer::from_seconds(3.0, TimerMode::Once),
                });
            }
        }
    }
}

pub fn enemy_destroyed(
    mut commands: Commands,
    query: Query<(Entity, &Enemy)>,
    mut player: ResMut<Player>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    children: Query<&Children>,
    mut collision_group_query: Query<&mut CollisionGroups>,
    enemy_health_query: Query<&EnemyHealth>,
) {
    for (entity, enemy) in query.iter() {
        if enemy.health <= 0.0 {
            player.score += enemy.score;

            // find an animation player among the descendants of the thing, there's only one in my case
            for entity in children.iter_descendants(entity) {
                if let Ok(mut animation_player) = animation_players.get_mut(entity) {
                    animation_player.play_with_transition(
                        animations.0[1].clone_weak(),
                        Duration::from_millis(250),
                    );
                }
                // despwan enemy helath bars, workaround if there are still health bars left
                if let Ok(_enemy_health) = enemy_health_query.get(entity) {
                    commands.entity(entity).despawn_recursive();
                }
            }

            commands.entity(entity).remove::<Enemy>();
            // despawn the enemy after 5 seconds
            commands.entity(entity).insert(DespawnTimer {
                timer: Timer::from_seconds(5.0, TimerMode::Once),
            });
            if let Ok(mut groups) = collision_group_query.get_mut(entity) {
                groups.memberships = Group::GROUP_5;
            };
        }
    }
}

pub fn despawn_enemy(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnTimer)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn enemy_health(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Enemy, &Children)>,
    enemy_health_query: Query<&EnemyHealth>,
    asset_server: Res<AssetServer>,
) {
    for (entity, enemy, children) in enemy_query.iter() {
        // keep track of how many health bars are spawned
        let mut health_count = 0;
        for child in children.iter() {
            if let Ok(_enemy_health) = enemy_health_query.get(*child) {
                health_count += 1;

                if (enemy.health as i32) + 1 < health_count {
                    commands.entity(*child).despawn_recursive();
                }
            }
        }

        commands
            .get_entity(entity)
            .unwrap()
            .with_children(|parent| {
                for health in 1..enemy.health as i32 + 1 - health_count {
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
