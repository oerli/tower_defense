use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::enemy::components::Enemy;

pub fn hit_target(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut enemy_query: Query<&mut Enemy>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                println!(
                    "Collision started between bodies with handles {:?} and {:?}",
                    entity1, entity2
                );
                enemy_query.get_mut(*entity2).ok().map(|mut enemy| {
                    enemy.health -= 1;
                    if enemy.health <= 0 {
                        commands.entity(*entity2).despawn();
                    }
                    // commands.entity(*entity1).despawn_recursive();
                });
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                println!(
                    "Collision stopped between bodies with handles {:?} and {:?}",
                    entity1, entity2
                );
            }
        }
        // println!("Received collision event: {:?}", collision_event);
    }
}
