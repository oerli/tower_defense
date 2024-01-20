use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Defense {
    pub targets: VecDeque<Entity>,
    pub damage: i32,
    pub shooting_timer: Timer,
}

#[derive(Component)]
pub struct Weapon;