use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::*;
use crate::enemy::components::*;

use super::components::*;

pub fn bullet_expired(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn hit_enemy(
    mut commands: Commands,
    mut enemy_query: Query<&mut Enemy>,
    bullet_query: Query<(Entity, &Bullet)>,
    rapier_context: Res<RapierContext>,
) {
    for (bullet_entity, bullet) in bullet_query.iter() {
        for collider_pair in rapier_context.contacts_with(bullet_entity) {
            let other_entity = if collider_pair.collider1() == bullet_entity {
                collider_pair.collider2()
            } else {
                collider_pair.collider1()
            };

            if let Ok(mut enemy) = enemy_query.get_mut(other_entity) {
                enemy.health -= bullet.damage;
                commands.entity(bullet_entity).despawn_recursive();

                // only count first hit on an enemy
                break;
            }
        }
    }
}
