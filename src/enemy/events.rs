use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{bullet::components::Bullet, defense::components::Defense, enemy::components::Enemy};

use super::components::*;

pub fn enemy_contact(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut defense_query: Query<&mut Defense>,
    mut enemy_query: Query<&mut Enemy>,
    bullet_query: Query<&Bullet>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                println!(
                    "Collision started between bodies with handles {:?} and {:?}",
                    entity1, entity2
                );

                bullet_query
                    .get(*entity1)
                    .ok()
                    .map(|bullet| {
                        info!("bullet hit enemy");

                        enemy_query.get_mut(*entity2).ok().map(|mut enemy| {
                            enemy.health -= bullet.damage;
                            info!("enemy health: {:?}", enemy.health);
                        });

                        commands.entity(*entity1).despawn_recursive();
                    });

                enemy_query.get_mut(*entity2).ok().map(|_enemy| {
                    info!("enemy went into range");
                    defense_query.get_mut(*entity1).ok().map(|mut defense| {
                        info!("in defense target");
                        defense.targets.push(*entity2);
                    });
                });

                bullet_query
                    .get(*entity2)
                    .ok()
                    .map(|bullet| {
                        info!("bullet hit enemy");

                        enemy_query.get_mut(*entity1).ok().map(|mut enemy| {
                            enemy.health -= bullet.damage;
                            info!("enemy health: {:?}", enemy.health);
                        });

                        commands.entity(*entity2).despawn_recursive();
                    });

                enemy_query.get_mut(*entity1).ok().map(|_enemy| {
                    info!("enemy went into range");
                    defense_query.get_mut(*entity2).ok().map(|mut defense| {
                        info!("in defense target");
                        defense.targets.push(*entity1);
                    });
                });
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                println!(
                    "Collision stopped between bodies with handles {:?} and {:?}",
                    entity1, entity2
                );
                enemy_query.get_mut(*entity2).ok().map(|_enemy| {
                    info!("enemy went out of range");
                    defense_query.get_mut(*entity1).ok().map(|mut defense| {
                        info!("left defense target");
                        defense.targets.retain(|&x| x != *entity2);
                    });
                });

                enemy_query.get_mut(*entity1).ok().map(|_enemy| {
                    info!("enemy went out of range");
                    defense_query.get_mut(*entity2).ok().map(|mut defense| {
                        info!("left defense target");
                        defense.targets.retain(|&x| x != *entity1);
                    });
                });
            }
        }
    }
}
