use bevy::prelude::*;

pub mod components;
mod systems;
pub mod resources;

use systems::*;
use resources::*;

use crate::GameState;

pub struct DefensePlugin;

impl Plugin for DefensePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<DefenseSelection>()
            // Systems
            .add_systems(Update, defense_shooting.run_if(in_state(GameState::Playing)))
            .add_systems(Update, weapon_rotation.run_if(in_state(GameState::Playing)));
    }
}
