use bevy::prelude::*;

pub mod components;
mod systems;
pub mod resources;
pub mod events;

use systems::*;
use resources::*;
use events::*;

use crate::{GameState, BuildEvent};

pub struct DefensePlugin;

impl Plugin for DefensePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<DefenseSelection>()
            // Systems
            .add_systems(Update, defense_shooting.run_if(in_state(GameState::Playing)))
            .add_systems(Update, weapon_rotation.run_if(in_state(GameState::Playing)))
            .add_systems(Update, spawn_defense.run_if(on_event::<BuildEvent>()));
    }
}
