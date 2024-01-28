use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

use crate::{BuildEvent, GameState};

pub struct DefensePlugin;

impl Plugin for DefensePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<DefenseSelection>()
            // Events
            .add_event::<RangeEvent>()
            .add_systems(Update, range_event.run_if(on_event::<RangeEvent>()))
            // Systems
            .add_systems(
                Update,
                defense_shooting.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, spawn_defense.run_if(on_event::<BuildEvent>()));
    }
}
