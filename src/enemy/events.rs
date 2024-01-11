use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{bullet::components::Bullet, defense::components::Defense, enemy::components::Enemy};

pub fn enemy_contact(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut defense_query: Query<&mut Defense>,
    mut enemy_query: Query<&mut Enemy>,
    bullet_query: Query<&Bullet>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                bullet_query.get(*entity1).ok().map(|bullet| {
                    enemy_query.get_mut(*entity2).ok().map(|mut enemy| {
                        enemy.health -= bullet.damage;
                    });

                    commands.entity(*entity1).despawn_recursive();
                });

                enemy_query.get_mut(*entity2).ok().map(|_enemy| {
                    defense_query.get_mut(*entity1).ok().map(|mut defense| {
                        defense.targets.push(*entity2);
                    });
                });

                bullet_query.get(*entity2).ok().map(|bullet| {
                    enemy_query.get_mut(*entity1).ok().map(|mut enemy| {
                        enemy.health -= bullet.damage;
                    });

                    commands.entity(*entity2).despawn_recursive();
                });

                enemy_query.get_mut(*entity1).ok().map(|_enemy| {
                    defense_query.get_mut(*entity2).ok().map(|mut defense| {
                        defense.targets.push(*entity1);
                    });
                });
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                enemy_query.get(*entity2).ok().map(|_enemy| {
                    defense_query.get_mut(*entity1).ok().map(|mut defense| {
                        defense.targets.retain(|&x| x != *entity2);
                    });
                });

                enemy_query.get(*entity1).ok().map(|_enemy| {
                    defense_query.get_mut(*entity2).ok().map(|mut defense| {
                        defense.targets.retain(|&x| x != *entity1);
                    });
                });
            }
        }
    }
}
