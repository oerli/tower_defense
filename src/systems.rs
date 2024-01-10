use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_rapier3d::prelude::*;

use crate::events::*;

pub fn setup_graphics(mut commands: Commands) {
    // add light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.,
    });

    // create the camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            button_orbit: MouseButton::Middle,
            ..Default::default()
        },
    ));
}

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // create the ground
    for x in -8..8 {
        for y in -8..8 {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 0.1, 1.0))),
                    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                    transform: Transform::from_xyz(x as f32, -0.6, y as f32),
                    ..default()
                },
                PickableBundle::default(),
                Collider::cuboid(0.50, 0.05, 0.5),
                CollisionGroups::new(Group::GROUP_4, Group::all()),
                On::<Pointer<Click>>::send_event::<BuildEvent>(),
            ));
        }
    }
}
