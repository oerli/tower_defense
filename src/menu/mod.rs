use bevy::prelude::*;

mod layouts;

use layouts::*;

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Update, tower_selection.run_if(in_state(GameState::RoundEnded)))
            .add_systems(Update, show_controls)
            .add_systems(Update, high_scores.run_if(in_state(GameState::GameOver)));
    }
}
