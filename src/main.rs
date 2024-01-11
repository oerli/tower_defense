use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_rapier3d::prelude::*;

mod events;
mod systems;
mod components;

mod bullet;
mod defense;
mod enemy;
mod player;
mod level;

use events::*;
use systems::*;

use bullet::BulletPlugin;
use defense::DefensePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use level::LevelPlugin;

fn main() {
    App::new()
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(DefaultPickingPlugins.build().disable::<RaycastBackend>())
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(DefensePlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(LevelPlugin)
        // Events
        .add_event::<BuildEvent>()
        // Systems
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, build_event.run_if(on_event::<BuildEvent>()))
        .add_systems(Update, update_text)
        .run();
}
