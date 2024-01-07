use bevy::{prelude::*, utils::FloatOrd};
use bevy_rapier3d::prelude::*;

use super::components::*;

use crate::{
    bullet::{self, components::*},
    enemy::components::*,
};

pub fn setup_defense(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
                material: materials.add(Color::rgb(0.3, 0.4, 0.5).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            // RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
            Defense {
                target: None,
                shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::ball(3.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
            ));
        });
}

pub fn defense_shooting(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Defense, &GlobalTransform)>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (entity, mut defense, transform) in query.iter_mut() {
        defense.shooting_timer.tick(time.delta());

        if defense.shooting_timer.finished() {
            if let Some(target) = defense.target{
            if let Ok(enemy) = enemy_query.get(target) {
                if let Some(target) = defense.target {

                    commands.spawn((
                        PbrBundle {
                            mesh: meshes
                                .add(Mesh::try_from(shape::Box::new(0.2, 0.2, 0.2)).unwrap()),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_xyz(0.0, 1.0, 0.0),
                            ..default()
                        },
                        RigidBody::Dynamic,
                        Collider::cuboid(0.1, 0.1, 0.1),
                        Velocity {
                            linvel: Vec3::new(0.0, 1.0, 0.0),
                            angvel: Vec3::new(0.0, 0.0, 0.0),
                        },
                        Bullet::new(enemy.translation(), 1.0),
                        ActiveEvents::COLLISION_EVENTS,
                        CollisionGroups::new(Group::GROUP_1, Group::GROUP_3),
                        // Lifetime {
                        //     timer: Timer::from_seconds(10.0, TimerMode::Once),
                        // },
                    ));
                }
            }
        }
        }
    }
}
