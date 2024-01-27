use bevy::prelude::*;
use super::components::*;

#[derive(Resource, Default)]
pub struct LevelHandle(pub Handle<Level>);

#[derive(Resource, Default)]
pub struct RoundHandle(pub Handle<Round>);

#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub level: Option<Level>,
    pub index: usize,
}

#[derive(Resource, Default)]
pub struct CurrentRound {
    pub round: Option<Round>,
    pub index: usize,
    pub timer: Timer,
}
