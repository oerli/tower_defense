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
    mut weapon_query: Query<(&Parent, &mut Transform), With<Weapon>>,
    defense_query: Query<&Defense>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
) {
    for (defense_entity, mut transform) in weapon_query.iter_mut() {
        if let Ok(defense) = defense_query.get(defense_entity.get()) {
            if let Some(target) = defense.targets.front() {
                if let Ok(enemy_transform) = enemy_query.get(*target) {
                    // transform.look_at(enemy_transform.translation(), Vec3::Y);

                    let direction = enemy_transform.translation() - transform.translation;
                    let angle = direction.y.atan2(direction.x);
                    let rotation = Quat::from_rotation_y(angle);

                    transform.rotation = rotation;

                }
            }
        }
    }
}