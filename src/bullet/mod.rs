use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::GameState;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Update, bullet_expired.run_if(in_state(GameState::Playing)))
            .add_systems(Update, hit_enemy.run_if(in_state(GameState::Playing)));
    }
}
