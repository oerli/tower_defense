use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Defense {
    pub target: Option<Entity>,
    pub shooting_timer: Timer,   
}
