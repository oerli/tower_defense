use bevy::prelude::*;

mod systems;
pub mod components;

use systems::*;

use crate::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(PostStartup, setup_level)
            .add_systems(Update, spawn_enemies.run_if(in_state(GameState::Playing)));
    }
}
