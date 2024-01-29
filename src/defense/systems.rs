use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;

use crate::bullet::components::*;
use crate::components::*;
use crate::enemy::components::*;

pub fn defense_shooting(
    weapon_query: Query<(&GlobalTransform, &Parent, &Weapon)>,
    mut defense_query: Query<&mut Defense>,
    transform_query: Query<&GlobalTransform, With<Enemy>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (weapon_transform, parent, weapon) in weapon_query.iter() {
        if let Ok(mut defense) = defense_query.get_mut(parent.get()) {
            if let Some(target) = defense.targets.front() {
                // check time if expired later, in case target is not anymore in range

                if let Ok(enemy_transform) = transform_query.get(*target) {
                    // wait time is finished to shoot is finished
                    defense.shooting_timer.tick(time.delta());
                    if !defense.shooting_timer.finished() {
                        continue;
                    }

                    // direction to enemy
                    let direction = enemy_transform.translation() - weapon_transform.translation();
                    // add PI for a 180 degree rotation
                    let rotation_angle = direction.x.atan2(direction.z) + PI;

                    // choose how bullet should look like
                    let (bullet_mesh, bullet_color) = match weapon {
                        Weapon::Cannon => (
                            Mesh::try_from(shape::Icosphere {
                                radius: 0.1,
                                subdivisions: 8,
                            })
                            .unwrap(),
                            Color::BLACK,
                        ),
                        Weapon::Archer => (
                            Mesh::try_from(shape::Icosphere {
                                radius: 0.03,
                                subdivisions: 8,
                            })
                            .unwrap(),
                            Color::DARK_GRAY,
                        ),
                        Weapon::Ballista => (
                            Mesh::try_from(shape::Box {
                                min_x: -0.02,
                                min_y: -0.02,
                                min_z: -0.3,
                                max_x: 0.02,
                                max_y: 0.02,
                                max_z: 0.3,
                            })
                            .unwrap(),
                            Color::MAROON,
                        ),
                    };

                    // shoot at enemy
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(bullet_mesh),
                            material: materials.add(bullet_color.into()),
                            transform: Transform::from_translation(weapon_transform.translation())
                                .with_rotation(Quat::from_rotation_y(rotation_angle)),
                            ..default()
                        },
                        RigidBody::Dynamic,
                        Collider::cuboid(0.1, 0.1, 0.1),
                        ExternalImpulse {
                            impulse: direction * 0.05,
                            torque_impulse: Vec3::new(0.0, 0.0, 0.0),
                        },
                        Bullet::new(enemy_transform.translation(), 1.0, defense.damage),
                        AudioBundle {
                            source: asset_server.load("sounds/shoot.ogg"),
                            settings: PlaybackSettings::ONCE.with_spatial(true),
                        },
                        CollisionGroups::new(Group::GROUP_1, Group::GROUP_3 | Group::GROUP_4),
                        Lifetime {
                            timer: Timer::from_seconds(1.0, TimerMode::Once),
                        },
                    ));
                } else {
                    // target does not exist anymore
                    defense.targets.pop_front();
                }
            }
        }
    }
}

pub fn weapon_rotation(
    mut weapon_query: Query<(&mut Transform, &GlobalTransform, &Parent), With<Weapon>>,
    defense_query: Query<&Defense>,
    transform_query: Query<&GlobalTransform>,
) {
    for (mut weapon_transform, weapon_global_transform, parent) in weapon_query.iter_mut() {
        if let Ok(defense) = defense_query.get(parent.get()) {
            if let Some(target) = defense.targets.front() {
                if let Ok(enemy_transform) = transform_query.get(*target) {
                    // look at enemy
                    let direction =
                        enemy_transform.translation() - weapon_global_transform.translation();
                    // add PI for a 180 degree rotation
                    let rotation_angle = direction.x.atan2(direction.z) + PI;
                    weapon_transform.rotation = Quat::from_rotation_y(rotation_angle);
                }
            }
        }
    }
}
