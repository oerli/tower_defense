use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_rapier3d::prelude::*;

pub fn setup_graphics(mut commands: Commands) {
    // Add light to the scene.
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.,
    });

    // Add a camera so we can see the debug-render.
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
    // Create the ground.
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(15.0, 0.1, 15.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -0.6, 0.0),
            ..default()
        },
        Collider::cuboid(7.5, 0.05, 7.5),
        CollisionGroups::new(Group::GROUP_4, Group::all()),
    ));

}
