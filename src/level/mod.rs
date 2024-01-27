use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

pub mod components;
mod resources;
mod systems;

use components::*;
use systems::*;
use resources::*;

use crate::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(TomlAssetPlugin::<Level>::new(&["level.toml"]))
            .add_plugins(TomlAssetPlugin::<Round>::new(&["round.toml"]))
            // Systems
            .add_systems(Startup, load_levels)
            .add_systems(Startup, load_rounds)
            .add_systems(Update, setup_level)
            .add_systems(Update, setup_round)
            .add_systems(Update, spawn_enemies.run_if(in_state(GameState::Playing)));
    }
}
