use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Tile;