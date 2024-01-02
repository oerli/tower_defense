use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .add_systems(Update, bullet_movement);
    }
}
