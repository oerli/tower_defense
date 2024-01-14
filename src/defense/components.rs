use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Defense {
    pub targets: Vec<Entity>,
    pub damage: i32,
    pub shooting_timer: Timer,
}
