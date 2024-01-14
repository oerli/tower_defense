use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::level::components::*;
use crate::player::resources::*;
use crate::resources::*;

use super::components::*;

// TODO: review movement system
pub fn enemy_movement(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Enemy,
        &mut Velocity,
        &GlobalTransform,
        &mut Transform,
    )>,
    query_level: Query<&Level>,
    mut player: ResMut<Player>,
) {
    let level = query_level.get_single().unwrap();

    for (entity, mut enemy, mut velocity, position, mut transform) in query.iter_mut() {
        if enemy.waypoint < level.waypoints.len() && enemy.health > 0 {
            transform.look_at(level.waypoints[enemy.waypoint], Vec3::ZERO);
            // TODO: dirty hack to rotate the enemy 180 degrees
            transform.rotate(Quat::from_rotation_y(PI));

            let mut direction = level.waypoints[enemy.waypoint] - position.translation();
            direction.y = 0.0;
            let distance = direction.length();

            if distance < 0.5 {
                enemy.waypoint += 1;
            } else {
                direction = direction.normalize();
                velocity.linvel += direction * enemy.speed;
            }
        } else if enemy.health > 0 {
            // enemy reached goal
            player.lives -= 1;
            commands.entity(entity).despawn();
        }
    }
}

pub fn enemy_destroyed(
    mut commands: Commands,
    query: Query<(Entity, &Enemy)>,
    mut player: ResMut<Player>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    children: Query<&Children>
) {
    for (entity, enemy) in query.iter() {
        if enemy.health <= 0 {
            player.score += enemy.score;

            // find an animation player among the descendants of the thing, there's only one in my case
            for entity in children.iter_descendants(entity) {
                if let Ok(mut animation_player) = animation_players.get_mut(entity) {
                    animation_player.play_with_transition(animations.0[1].clone_weak(), Duration::from_millis(250));
                }
            }

            // TODO: add a delay before despawning the enemy
            commands.entity(entity).remove::<Enemy>();
        }
    }
}
