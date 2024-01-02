use bevy::prelude::*;

mod systems;
use systems::*;

pub struct DefensePlugin;

impl Plugin for DefensePlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Startup, setup_defense)
            .add_systems(Update, defense_shooting);
    }
}