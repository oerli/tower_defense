use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;

use crate::{bullet::components::*, components::*, enemy::components::*};

// unordered defense shooting and weapon moving to enemy
pub fn defense_shooting(
    rapier_context: Res<RapierContext>,
    mut defense_query: Query<(Entity, &mut Defense, &GlobalTransform, &Children)>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
    mut transform_query: Query<&mut Transform, With<Weapon>>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (defense_entity, mut defense, defense_transform, children) in defense_query.iter_mut() {
        for (collider1, collider2, intersecting) in
            rapier_context.intersections_with(defense_entity)
        {
            if !intersecting {
                continue;
            }

            let other_entity = if collider1 == defense_entity {
                collider2
            } else {
                collider1
            };

            if let Ok(enemy_transform) = enemy_query.get(other_entity) {
                // look at enemy
                let direction = enemy_transform.translation() - defense_transform.translation();
                // add PI for a 180 degree rotation
                let rotation_angle = direction.x.atan2(direction.z) + PI;
                for child in children.iter() {
                    if let Ok(mut child_transform) = transform_query.get_mut(*child) {
                        child_transform.rotation = Quat::from_rotation_y(rotation_angle);
                    }
                }

                // wait time is finished to shoot is finished
                defense.shooting_timer.tick(time.delta());

                if !defense.shooting_timer.finished() {
                    continue;
                }

                // shoot at enemy
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
                            defense_transform.translation() + Vec3::new(0.0, 0.5, 0.0),
                        ),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Collider::cuboid(0.1, 0.1, 0.1),
                    ExternalImpulse {
                        impulse: (enemy_transform.translation() - defense_transform.translation())
                            * 0.05,
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
            }

            // only target at first enemy
            break;
        }
    }
}
