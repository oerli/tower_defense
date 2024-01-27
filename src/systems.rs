use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_rapier3d::prelude::*;

use crate::player::resources::*;
use crate::GameState;
use crate::resources::*;

pub fn setup_graphics(mut commands: Commands, player: Res<Player>) {
    // add light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.15,
    });

    // orange point light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 6.0, 0.0),
        point_light: PointLight {
            intensity: 3600.0,
            color: Color::Rgba {
                red: 1.0,
                green: 0.65,
                blue: 0.45,
                alpha: 1.0,
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // create the camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            button_orbit: MouseButton::Middle,
            focus: Vec3::new(0.0, 1.0, 2.0),
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
            TextSection::new(" Credits: ", TextStyle::default()),
            TextSection::new(format!("{}", player.credits), TextStyle::default()),
        ]),
        PlayerText,
    ));
}

pub fn setup_physics(mut commands: Commands) {
    // create the ground
    commands.spawn((
        Collider::cuboid(9.0, 0.2, 9.0),
        CollisionGroups::new(Group::GROUP_4, Group::all()),
        TransformBundle::from(Transform::from_xyz(-0.5, -0.6, -0.5)),
        Pickable::IGNORE,
    ));    
}

#[derive(Component)]
pub struct PlayerText;

pub fn update_text(mut query: Query<&mut Text, With<PlayerText>>, player: Res<Player>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", player.level);
        text.sections[3].value = format!("{}", player.lives);
        text.sections[5].value = format!("{}", player.score);
        text.sections[7].value = format!("{}", player.credits);
    }
}

pub fn change_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *game_state.get() == GameState::Playing {
            next_game_state.set(GameState::Paused);
        } else if *game_state.get() == GameState::Paused {
            next_game_state.set(GameState::Playing);
        }
    }
}

// once the scene is loaded, start the animation
pub fn play_animations(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}
