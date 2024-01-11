use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::level::components::*;
use crate::player::resources::*;

use super::components::*;

pub fn enemy_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Enemy, &mut Velocity, &GlobalTransform)>,
    query_level: Query<&Level>,
    mut player: ResMut<Player>,
) {
    let level = query_level.get_single().unwrap();

    for (entity, mut enemy, mut velocity, position) in query.iter_mut() {
        if enemy.waypoint < level.waypoints.len() {
            let mut direction = level.waypoints[enemy.waypoint] - position.translation();
            direction.y = 0.0;
            let distance = direction.length();

            if distance < 0.5 {
                enemy.waypoint += 1;
            } else {
                direction = direction.normalize();
                velocity.linvel += direction * enemy.speed;
            }
        } else {
            // enemy reached goal
            player.lives -= 1;
            commands.entity(entity).despawn();
        }
    }
}

pub fn enemy_destroyed(
    mut commands: Commands,
    mut query: Query<(Entity, &Enemy)>,
    mut player: ResMut<Player>,
) {
    for (entity, enemy) in query.iter_mut() {
        if enemy.health <= 0 {
            player.score += enemy.score;
            commands.entity(entity).despawn();
        }
    }
}
