use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub target: Vec3,
    pub speed: f32,
    pub damage: f32,
}

impl Bullet {
    pub fn new(target: Vec3, speed: f32, damage: f32) -> Self {
        Bullet {
            target,
            speed,
            damage,
        }
    }
}
