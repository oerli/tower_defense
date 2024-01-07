use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::enemy::components::Enemy;

use super::components::*;

pub fn bullet_hit(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut enemy_query: Query<&mut Enemy>,
    bullet_query: Query<Entity, With<Bullet>>,
) {
    for collision_event in collision_events.read() {
        // println!("Collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                // println!(
                //     "Collision started between bodies with handles {:?} and {:?}",
                //     entity1, entity2
                // );

                bullet_query.get(*entity2).ok().map(|entity| {
                    // println!("Despawning bullet: {:?}", entity);
                    commands.entity(entity).despawn_recursive();

                    enemy_query.get_mut(*entity1).ok().map(|mut enemy| {
                        enemy.health -= 1;
                        // println!("Enemy health: {:?}", enemy.health);
                    });
                });
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                // println!(
                //     "Collision stopped between bodies with handles {:?} and {:?}",
                //     entity1, entity2
                // );
            }
        }
    }
}
