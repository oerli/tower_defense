use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;

pub mod components;
mod systems;
mod events;

use systems::*;
// use events::*;

pub struct DefensePlugin;

impl Plugin for DefensePlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Startup, setup_defense)
            .add_systems(Update, defense_shooting);
    }
}
