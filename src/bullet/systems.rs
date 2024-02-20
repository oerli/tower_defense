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
    mut enemy_query: Query<(Entity, &mut Enemy)>,
    bullet_query: Query<(Entity, &Bullet)>,
    rapier_context: Res<RapierContext>,
) {
    for (bullet_entity, bullet) in bullet_query.iter() {
        for (enemy_entity, mut enemy) in enemy_query.iter_mut() {
            // check if bullet hits enemy
            if let Some(contact_pair) = rapier_context.contact_pair(bullet_entity, enemy_entity) {
                if contact_pair.has_any_active_contacts() {
                    enemy.health -= bullet.damage;
                    commands.entity(bullet_entity).despawn_recursive();

                    // only count first hit on an enemy
                    break;
                }
            }
        }
    }
}
