use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy {
    pub speed: f32,
    pub health: i32,
}
