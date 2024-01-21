use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod components;
pub mod events;
mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

use crate::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemyPath>()
            // Events
            .add_event::<SpawnEnemyEvent>()
            .add_systems(Update, enemy_contact.run_if(on_event::<CollisionEvent>()))
            .add_systems(Update, spawn_enemy.run_if(on_event::<SpawnEnemyEvent>()))
            // Systems
            .add_systems(Update, enemy_destroyed.run_if(in_state(GameState::Playing)))
            .add_systems(Update, enemy_movement.run_if(in_state(GameState::Playing)))
            .add_systems(Update, despawn_enemy.run_if(in_state(GameState::Playing)))
            .add_systems(Update, enemy_health.run_if(in_state(GameState::Playing)));
    }
}
