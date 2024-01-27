use bevy::prelude::*;
use super::components::*;

#[derive(Resource)]
pub struct LevelHandle(pub Handle<Level>);

#[derive(Resource)]
pub struct RoundHandle(pub Handle<Round>);

impl Default for RoundHandle {
    fn default() -> Self {
        RoundHandle(Handle::default())
    }
}