use bevy::{prelude::*, utils::FloatOrd};
use bevy_rapier3d::prelude::*;

use super::components::*;

use crate::{bullet::components::*, enemy::components::*};

pub fn setup_defense(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: materials.add(Color::rgb(0.3, 0.4, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Defense {
            range: 3.0,
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
    ));
}

pub fn defense_shooting(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Defense, &GlobalTransform)>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (entity, mut defense, transform) in query.iter_mut() {
        defense.shooting_timer.tick(time.delta());

        if defense.shooting_timer.finished() {
            let bullet_start = transform.translation();

            let direction = enemies
                .iter()
                .filter(|enemy_transform| {
                    Vec3::distance(enemy_transform.translation(), bullet_start) < defense.range
                })
                .min_by_key(|enemy_transform| {
                    FloatOrd(Vec3::distance(enemy_transform.translation(), bullet_start))
                })
                .map(|closest_target| closest_target.translation() - bullet_start);

            if let Some(direction) = direction {
                // let (model, bullet) = tower_type.get_bullet(direction, &bullet_assets);
                commands.entity(entity).with_children(|commands| {
                    info!("Spawning bullet");
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes
                                .add(Mesh::try_from(shape::Box::new(0.01, 0.01, 0.01)).unwrap()),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_xyz(0.0, 4.0, 0.0),
                            ..default()
                        },
                        RigidBody::Dynamic,
                        Collider::ball(0.5),
                        Restitution::coefficient(0.7),
                        Lifetime {
                            timer: Timer::from_seconds(10.0, TimerMode::Once),
                        },
                    ));
                });
            }
        }
    }
}
