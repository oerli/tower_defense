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

#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub level_index: usize,
    pub running: bool,
}

#[derive(Resource, Default)]
pub struct CurrentRound {
    pub round: Option<Round>,
    pub index: usize,
    pub timer: Timer,
}
