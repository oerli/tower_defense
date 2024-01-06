use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{bullet::components::Bullet, enemy::components::Enemy};

// pub fn modify_collider_active_events(mut active_events: Query<&mut ActiveEvents>, collision_groups: Query<&CollisionGroups>) {
//     for mut active_events in active_events.iter_mut() {
//         // // *active_events = ActiveEvents::COLLISION_EVENTS;
//         // if collision_groups.get(active_events.).unwrap().membership == 1 {
//         //     *active_events = ActiveEvents::CONTACT_EVENTS;
//         // }

//     }
// }

/* A system that displays the events. */
pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, flags) => {
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
                        // Lifetime {
                        //     timer: Timer::from_seconds(10.0, TimerMode::Once),
                        // },
                    ));
                });
            }
            CollisionEvent::Stopped(entity1, entity2, flags) => {
                println!(
                    "Collision stopped between bodies with handles {:?} and {:?}",
                    entity1, entity2
                );
            }
        }
        // println!("Received collision event: {:?}", collision_event);
    }
}
