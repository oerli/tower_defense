use bevy::prelude::*;

use super::resources::EnemyPath;
use crate::components::Health;

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
        Health { value: 10 },
    ));
}

pub fn enemy_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Health)>,
    mut path: ResMut<EnemyPath>,
) {
    for (entity, mut transform, health) in query.iter_mut() {
        if path.waypoints.len() > 0 && health.value > 0 {
            let mut position = transform.translation;
            let mut direction = path.waypoints[0] - position;
            direction.y = 0.0;
            let distance = direction.length();
            if distance < 0.1 {
                path.waypoints.remove(0);
            } else {
                direction = direction.normalize();
                position += direction * 0.05;
                transform.translation = position;
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}
