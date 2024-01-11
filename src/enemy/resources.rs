use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyPath {
    pub waypoints: Vec<Vec3>,
}

impl Default for EnemyPath {
    fn default() -> Self {
        EnemyPath {
            waypoints: vec![
                Vec3::new(-7.0, 0.0, -8.0),
                Vec3::new(-6.0, 0.0, -8.0),
                Vec3::new(-5.0, 0.0, -8.0),
                Vec3::new(-4.0, 0.0, -8.0),
                Vec3::new(-4.0, 0.0, -7.0),
                Vec3::new(-4.0, 0.0, -6.0),
                Vec3::new(-4.0, 0.0, -5.0),
                Vec3::new(-4.0, 0.0, -4.0),
                Vec3::new(-3.0, 0.0, -4.0),
                Vec3::new(-2.0, 0.0, -4.0),
                Vec3::new(-1.0, 0.0, -4.0),
                Vec3::new(0.0, 0.0, -4.0),
                Vec3::new(0.0, 0.0, -3.0),
                Vec3::new(0.0, 0.0, -2.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(2.0, 0.0, 0.0),
                Vec3::new(3.0, 0.0, 0.0),
                Vec3::new(4.0, 0.0, 0.0),
                Vec3::new(4.0, 0.0, 1.0),
                Vec3::new(4.0, 0.0, 2.0),
                Vec3::new(4.0, 0.0, 3.0),
                Vec3::new(4.0, 0.0, 4.0),
                Vec3::new(5.0, 0.0, 4.0),
                Vec3::new(6.0, 0.0, 4.0),
                Vec3::new(7.0, 0.0, 4.0),
                Vec3::new(7.0, 0.0, 5.0),
                Vec3::new(7.0, 0.0, 6.0),
                Vec3::new(7.0, 0.0, 7.0),
            ],
        }
    }
}
