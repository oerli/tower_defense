use bevy::prelude::*;

#[derive(Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct Enemy {
    pub speed: f32,
    pub health: f32,
    pub score: i32,
    pub waypoint: usize,
}

#[derive(Component)]
pub struct EnemyHealth;

#[derive(Component)]
pub struct DespawnTimer {
    pub timer: Timer,
}
