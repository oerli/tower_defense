use bevy::prelude::*;
use super::components::*;

#[derive(Resource)]
pub struct LevelHandle(pub Handle<Level>);
