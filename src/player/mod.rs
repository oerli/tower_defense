use bevy::prelude::*;

pub mod resources;

use resources::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<Player>();
    }
}
