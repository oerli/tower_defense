use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;

use crate::{bullet::components::*, components::*, enemy::components::*};

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

        if defense.shooting_timer.finished() {
            if let Some(target) = defense.targets.get(0) {
                if let Ok(enemy) = enemy_query.get(*target) {
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes
                                .add(Mesh::try_from(shape::Box::new(0.2, 0.2, 0.2)).unwrap()),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_translation(
                                transform.translation() + Vec3::new(0.0, 0.5, 0.0),
                            ),
                            ..default()
                        },
                        RigidBody::Dynamic,
                        Collider::cuboid(0.1, 0.1, 0.1),
                        // Velocity {
                        //     linvel: enemy.translation() - transform.translation(),
                        //     angvel: Vec3::new(0.0, 0.0, 0.0),
                        // },
                        ExternalImpulse {
                            impulse: (enemy.translation() - transform.translation()) * 0.05,
                            torque_impulse: Vec3::new(0.1, 0.1, 0.1),
                        },
                        Bullet::new(enemy.translation(), 1.0, defense.damage),
                        CollisionGroups::new(Group::GROUP_1, Group::GROUP_3 | Group::GROUP_4),
                        Lifetime {
                            timer: Timer::from_seconds(1.0, TimerMode::Once),
                        },
                    ));
                }
            }
        }
    }
}
