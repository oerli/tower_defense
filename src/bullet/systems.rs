use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;

pub fn bullet_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut GlobalTransform, &mut Velocity, &Bullet)>,
) {
    for (entity, position, mut velocity, bullet) in query.iter_mut() {
        let mut direction = bullet.target - position.translation();

        let distance = direction.length();
        // if distance < 0.5 {
        //     commands.entity(entity).despawn_recursive();
        // } else {
            direction = direction.normalize();
            velocity.linvel += direction * bullet.speed;
        // }
    }
}
