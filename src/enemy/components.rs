use bevy::prelude::*;
use bevy::asset::Asset;
use serde::Deserialize;
use bevy::reflect::TypePath;

#[derive(Component, Deserialize, Asset, TypePath, Clone, Debug)]
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
