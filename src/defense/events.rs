use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{bullet::components::Bullet, enemy::components::Enemy};

pub fn enemy_contact(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                println!(
                    "Collision started between bodies with handles {:?} and {:?}",
                    entity1, entity2
                );
                enemy_query.get(*entity2).ok().map(|enemy_transform| {
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
                        Bullet::new(enemy_transform.translation(), 1.0),
                        CollisionGroups::new(Group::GROUP_2, Group::GROUP_2),
                        // Lifetime {
                        //     timer: Timer::from_seconds(10.0, TimerMode::Once),
                        // },
                    ));
                });
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                println!(
                    "Collision stopped between bodies with handles {:?} and {:?}",
                    entity1, entity2
                );
            }
        }
    }
}
