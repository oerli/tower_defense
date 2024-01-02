use bevy::prelude::*;

use super::components::*;

pub fn bullet_movement (
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Bullet)>,
) {
    for (entity, mut transform, bullet) in query.iter_mut() {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
        if transform.translation.x < -400.0 || transform.translation.x > 400.0 || transform.translation.y < -300.0 || transform.translation.y > 300.0 {
            commands.entity(entity).despawn();
        }
    }
}