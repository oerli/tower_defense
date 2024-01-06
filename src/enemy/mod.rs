use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemyPath>()
            // Systems
            .add_systems(Startup, setup_enemies)
            .add_systems(Update, enemy_movement);
    }
}
