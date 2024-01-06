use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::Health;

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
            transform: Transform::from_xyz(-5.0, 0.0, -5.0),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        },
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
        Health { value: 10 },
        Enemy { speed: 0.1 },
    ));
}

pub fn enemy_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &Enemy, &mut Velocity, &GlobalTransform)>,
    mut path: ResMut<EnemyPath>,
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
            commands.entity(entity).despawn();
        }
    }
}
