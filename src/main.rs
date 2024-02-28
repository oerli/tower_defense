use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_rapier3d::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::EguiPlugin;
use oxidized_navigation::{
    debug_draw::OxidizedNavigationDebugDrawPlugin, OxidizedNavigationPlugin, NavMeshSettings,
};

mod components;
pub mod events;
mod resources;
mod systems;

mod bullet;
mod defense;
mod enemy;
mod level;
mod menu;
mod player;

use events::*;
use resources::*;
use systems::*;

use bullet::BulletPlugin;
use defense::DefensePlugin;
use enemy::EnemyPlugin;
use level::LevelPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        // Plugins
        .add_plugins(DefaultPlugins)
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(EguiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(
            DefaultPickingPlugins
                .build()
                .disable::<RaycastBackend>()
                .disable::<DebugPickingPlugin>(),
        )
        .add_plugins(OxidizedNavigationPlugin::<Collider>::new(NavMeshSettings {
            cell_width: 0.25,
            cell_height: 0.1,
            tile_width: 100,
            world_half_extents: 250.0,
            world_bottom_bound: -100.0,
            max_traversable_slope_radians: (40.0_f32 - 0.1).to_radians(),
            walkable_height: 20,
            walkable_radius: 1,
            step_height: 3,
            min_region_area: 100,
            merge_region_area: 500,
            max_contour_simplification_error: 1.1,
            max_edge_length: 80,
            max_tile_generation_tasks: Some(9),
        }))
        .add_plugins(OxidizedNavigationDebugDrawPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(DefensePlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(MenuPlugin)
        // Resources
        .init_resource::<HoverHandler>()
        .insert_resource(RapierBackendSettings {
            require_markers: true,
        })
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
        .add_systems(Update, run_blocking_pathfinding)
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
