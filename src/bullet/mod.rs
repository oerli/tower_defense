use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod components;
mod systems;
mod events;

use systems::*;
use events::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Update, bullet_hit.run_if(on_event::<CollisionEvent>()))
            .add_systems(Update, bullet_movement);
    }
}
