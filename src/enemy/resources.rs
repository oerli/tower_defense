use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyPath {
    pub waypoints: Vec<Vec3>,
}

impl Default for EnemyPath {
    fn default() -> Self {
        EnemyPath {
            waypoints: vec![
                Vec3::new(-8.0, 0.0, -8.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(8.0, 0.0, 8.0),
            ],
        }
    }
}
