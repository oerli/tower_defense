use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::GameState;

pub struct DefensePlugin;

impl Plugin for DefensePlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Update, defense_shooting.run_if(in_state(GameState::Playing)))
            .add_systems(Update, weapon_rotation.run_if(in_state(GameState::Playing)));
    }
}
