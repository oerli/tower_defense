use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::resources::*;

use super::components::*;
use super::resources::*;

pub fn setup_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            transform: Transform::from_xyz(-8.0, 0.0, -8.0),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        },
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::new(
            Group::GROUP_3,
            Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_4,
        ),
        Enemy {
            speed: 0.1,
            health: 10,
        },
    ));
}

pub fn enemy_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Enemy, &mut Velocity, &GlobalTransform)>,
    mut path: ResMut<EnemyPath>,
    mut player: ResMut<Player>,
) {
    for (entity, enemy, mut velocity, position) in query.iter_mut() {
        if path.waypoints.len() > 0 {
            let mut direction = path.waypoints[0] - position.translation();
            direction.y = 0.0;
            let distance = direction.length();
            if distance < 0.5 {
                path.waypoints.remove(0);
            } else {
                direction = direction.normalize();
                velocity.linvel += direction * enemy.speed;
            }
        } else {
            // enemy reached goal
            player.score -= 1;
            commands.entity(entity).despawn();
        }
    }
}

pub fn enemy_destroyed(mut commands: Commands, mut query: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in query.iter_mut() {
        if enemy.health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
