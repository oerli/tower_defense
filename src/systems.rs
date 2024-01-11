use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_rapier3d::prelude::*;

use crate::{events::*, player::resources::*, components::*};

pub fn setup_graphics(mut commands: Commands, player: Res<Player>) {
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

    // show player score
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Level: ", TextStyle::default()),
            TextSection::new(format!("{}", player.level), TextStyle::default()),
            TextSection::new(" Lives: ", TextStyle::default()),
            TextSection::new(format!("{}", player.lives), TextStyle::default()),
            TextSection::new(" Score: ", TextStyle::default()),
            TextSection::new(format!("{}", player.score), TextStyle::default()),
        ]),
        PlayerText,
    ));
}

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // create the ground
    commands.spawn((
        Collider::cuboid(8.0, 0.05, 8.0),
        CollisionGroups::new(Group::GROUP_4, Group::all()),
        TransformBundle::from(Transform::from_xyz(-0.5, -0.6, -0.5)),
        Pickable::IGNORE,
    ));

    // create tiles
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
                CollisionGroups::new(Group::GROUP_5, Group::GROUP_5),
                On::<Pointer<Click>>::send_event::<BuildEvent>(),
                Tile,
            ));
        }
    }
}

#[derive(Component)]
pub struct PlayerText;

pub fn update_text(mut query: Query<&mut Text, With<PlayerText>>, player: Res<Player>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", player.level);
        text.sections[3].value = format!("{}", player.lives);
        text.sections[5].value = format!("{}", player.score);
    }
}
