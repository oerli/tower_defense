use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;

use crate::{bullet::components::*, components::*, enemy::components::*};

// TODO: might be a better way to do this
pub fn defense_shooting(
    mut commands: Commands,
    mut query: Query<(&mut Defense, &GlobalTransform)>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (mut defense, transform) in query.iter_mut() {
        defense.shooting_timer.tick(time.delta());

        if !defense.shooting_timer.finished() {
            continue;
        }

        // check if target is still alive
        loop {
            if let Some(target) = defense.targets.pop_front() {
                if let Ok(enemy) = enemy_query.get(target) {
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(
                                Mesh::try_from(shape::Icosphere {
                                    radius: 0.1,
                                    subdivisions: 8,
                                })
                                .unwrap(),
                            ),
                            material: materials.add(Color::BLACK.into()),
                            transform: Transform::from_translation(
                                transform.translation() + Vec3::new(0.0, 0.5, 0.0),
                            ),
                            ..default()
                        },
                        RigidBody::Dynamic,
                        Collider::cuboid(0.1, 0.1, 0.1),
                        ExternalImpulse {
                            impulse: (enemy.translation() - transform.translation()) * 0.05,
                            torque_impulse: Vec3::new(0.0, 0.0, 0.0),
                        },
                        Bullet::new(enemy.translation(), 1.0, defense.damage),
                        CollisionGroups::new(Group::GROUP_1, Group::GROUP_3 | Group::GROUP_4),
                        Lifetime {
                            timer: Timer::from_seconds(1.0, TimerMode::Once),
                        },
                    ));

                    defense.shooting_timer.reset();
                    // put back the target
                    defense.targets.push_front(target);
                    break;
                }
            } else {
                break;
            }
        }
    }
}

pub fn weapon_rotation(
    mut defense_query: Query<&mut Transform, With<Defense>>,
) {
    for mut transform in defense_query.iter_mut() {
        let target_position = Vec3::new(0.0, 0.0, 0.0);
        let direction = target_position - transform.translation;
        // add PI for a 180 degree rotation
        let rotation_angle = direction.x.atan2(direction.z) + PI;
        transform.rotation = Quat::from_rotation_y(rotation_angle);
    }
}
