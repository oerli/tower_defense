use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_rapier3d::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::EguiPlugin;

pub mod events;
mod systems;
mod components;
mod resources;

mod bullet;
mod defense;
mod enemy;
mod player;
mod level;
mod menu;

use events::*;
use resources::*;
use systems::*;

use bullet::BulletPlugin;
use defense::DefensePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use level::LevelPlugin;
use menu::MenuPlugin;

fn main() {
    App::new()
        // Plugins
        .add_plugins(DefaultPlugins)
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(EguiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(DefaultPickingPlugins.build().disable::<RaycastBackend>().disable::<DebugPickingPlugin>())
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(DefensePlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(MenuPlugin)
        // Resources
        .init_resource::<HoverHandler>()
        // Events
        .add_event::<BuildEvent>()
        .add_event::<OverEvent>()
        // .add_event::<OutEvent>()
        .add_systems(Update, hover_event.run_if(on_event::<OverEvent>()))
        // .add_systems(Update, out_event.run_if(on_event::<OutEvent>()))
        // State
        .add_state::<GameState>()
        // Systems
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, update_text)
        .add_systems(Update, change_game_state)
        .add_systems(Update, play_animations.run_if(in_state(GameState::Playing)))
        .run();
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Playing,
    Paused,
    #[default]
    RoundEnded,
    GameOver,
}