use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_rapier3d::prelude::*;

use oxidized_navigation::{
    debug_draw::{DrawPath, DrawNavMesh},
    query::{find_polygon_path, perform_string_pulling_on_path},
    NavMesh, NavMeshSettings, NavMeshAffector,
};

use crate::{ level::components::*, player::resources::*};
use crate::GameState;
use crate::resources::*;

pub fn setup_graphics(mut commands: Commands) {
    // add light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.15,
    });

    // orange point light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(8.0, 6.0, 8.0),
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
            transform: Transform::from_xyz(8.0, 10.0, 14.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            button_orbit: MouseButton::Middle,
            focus: Vec3::new(8.0, 1.0, 8.0),
            ..Default::default()
        },
        RapierPickable,
        SpatialListener::new(5.0),
    ));
}

pub fn setup_physics(mut commands: Commands) {

    let heights = vec![
             0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
             0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  3.0,  3.0,  3.0,  3.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  2.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0,
             0.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0,
             0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
         ];

    // create the ground
    commands.spawn((
        // Collider::cuboid(10.0, 0.2, 10.0),
        Collider::heightfield(heights, 16, 16, Vec3::new(16.0, 0.2, 16.0)),
        CollisionGroups::new(Group::GROUP_4, Group::all()),
        TransformBundle::from(Transform::from_xyz(7.5, 0.0, 7.5)),
        NavMeshAffector,
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


//
//  Blocking Pathfinding.
//  Press B to run.
//
//  Running pathfinding in a system.
//
pub fn run_blocking_pathfinding(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut draw_nav_mesh: ResMut<DrawNavMesh>,
    mut level_query: Query<&mut Level>,
) {
    if !keys.just_pressed(KeyCode::B) {
        return;
    }

    // draw_nav_mesh.0 = true;

    // Get the underlying nav_mesh.
    if let Ok(nav_mesh) = nav_mesh.get().read() {
        // let start_pos = Vec3::new(8.0, 0.2, 3.0);
        // let end_pos = Vec3::new(12.0, 0.2, 12.0);
        let start_pos = Vec3::new(8.0, 0.2, 3.0);
        let end_pos = Vec3::new(12.0, 0.2, 12.0);

        // commands.spawn((Collider::cuboid(1.0, 1.0, 1.0), TransformBundle::from_transform(
        //     Transform::from_translation(start_pos)
        // )));

        // commands.spawn((Collider::cuboid(1.0, 1.0, 1.0), TransformBundle::from_transform(
        //     Transform::from_translation(end_pos)
        // )));
        // Run pathfinding to get a polygon path.
        match find_polygon_path(
            &nav_mesh,
            &nav_mesh_settings,
            start_pos,
            end_pos,
            None,
            Some(&[1.0, 0.5]),
        ) {
            Ok(path) => {
                info!("Path found (BLOCKING): {:?}", path);

                // Convert polygon path to a path of Vec3s.
                match perform_string_pulling_on_path(&nav_mesh, start_pos, end_pos, &path) {
                    Ok(string_path) => {

                        for mut level in level_query.iter_mut() {
                            level.waypoints = Some(string_path.clone());
                        }

                        info!("String path (BLOCKING): {:?}", string_path);
                        commands.spawn(DrawPath {
                            timer: Some(Timer::from_seconds(4.0, TimerMode::Once)),
                            pulled_path: string_path,
                            color: Color::RED,
                        });
                    }
                    Err(error) => error!("Error with string path: {:?}", error),
                };
            }
            Err(error) => error!("Error with pathfinding: {:?}", error),
        }
    }
}