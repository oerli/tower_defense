use bevy::prelude::*;

use crate::components::*;

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
