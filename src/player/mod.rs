use bevy::prelude::*;

mod resources;

use resources::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems
            .init_resource::<Player>();
    }
}
