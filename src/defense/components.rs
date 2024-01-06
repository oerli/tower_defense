use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Defense {
    pub shooting_timer: Timer,
}
