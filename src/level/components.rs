use bevy::prelude::*;
use bevy::asset::Asset;
use serde::Deserialize;
use bevy::reflect::TypePath;

use crate::enemy::components::*;

#[derive(Component, Deserialize, Asset, TypePath, Debug)]
pub struct Level {
    pub waypoints: Option<Vec<Vec3>>,
    pub rounds: usize,
    pub last_level: bool,
    pub path: Vec<Vec<u8>>,
    pub heights: Vec<Vec<f32>>,
}

#[derive(Component, Deserialize, Asset, TypePath, Clone, Debug)]
pub struct Round {
    pub enemy: Enemy,
    pub enemy_count: i32,
    pub separation_time: f32,
}

#[derive(Component)]
pub struct Terrain;
