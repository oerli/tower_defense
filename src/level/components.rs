use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Level {
    pub enemies: i32,
    pub separation_timer: Timer,   
    pub waypoints: Vec<Vec3>,
}
