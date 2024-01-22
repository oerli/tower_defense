use bevy::prelude::*;

use crate::enemy::components::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Level {  
    pub waypoints: Vec<Vec3>,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Round {
    pub index: i32,
    pub enemy: Enemy,
    pub enemy_count: i32,
    pub separation_timer: Timer,
}
