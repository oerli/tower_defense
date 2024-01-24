use bevy::prelude::*;

#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub struct Defense {
    pub damage: f32,
    pub shooting_timer: Timer,
}

#[derive(Component)]
pub enum Weapon {
    Cannon,
    Ballista,
    Archer,
}
