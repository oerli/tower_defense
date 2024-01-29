use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub struct Defense {
    pub damage: f32,
    pub shooting_timer: Timer,
    // todo: use index instead of entity to have target at multiple defense towers
    pub targets: VecDeque<Entity>,
}

#[derive(Component, Clone)]
pub enum Weapon {
    Cannon,
    Ballista,
    Archer,
}

#[derive(Component)]
pub struct DefenseRange;
