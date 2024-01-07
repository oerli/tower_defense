use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_rapier3d::prelude::*;

mod systems;

mod enemy;
mod player;
mod defense;
mod bullet;

use systems::*;

use player::PlayerPlugin;
use enemy::EnemyPlugin;
use defense::DefensePlugin;
use bullet::BulletPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(DefensePlugin)
        .add_plugins(BulletPlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .run();
}
