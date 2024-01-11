use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod components;
mod resources;
mod systems;
mod events;

use resources::*;
use systems::*;
use events::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemyPath>()
            // Events
            .add_systems(Update, enemy_contact.run_if(on_event::<CollisionEvent>()))
            // Systems
            .add_systems(Update, enemy_destroyed)
            .add_systems(Update, enemy_movement);
    }
}
