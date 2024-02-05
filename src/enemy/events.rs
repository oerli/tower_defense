use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::enemy::components::*;

#[derive(Event)]
pub struct SpawnEnemyEvent {
    pub enemy: Enemy,
    pub position: Vec3,
}

pub fn spawn_enemy(
    mut enemy_events: EventReader<SpawnEnemyEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in enemy_events.read() {
        commands
            .spawn((
                RigidBody::KinematicPositionBased,
                // Collider::cuboid(0.5, 0.5, 0.5),
                // Velocity {
                //     linvel: Vec3::new(0.0, 0.0, 0.0),
                //     angvel: Vec3::new(0.0, 0.0, 0.0),
                // },
                Collider::cuboid(0.25, 0.5, 0.25),
                TransformBundle::from(Transform::from_translation(event.position)),
                VisibilityBundle::default(),
                KinematicCharacterController {
                    up: Vec3::Y,
                    max_slope_climb_angle: f32::to_radians(60.0),
                    min_slope_slide_angle: f32::to_radians(30.0),
                    snap_to_ground: Some(CharacterLength::Absolute(0.01)),
                    offset: CharacterLength::Absolute(0.01),
                    // translation: Some(event.position + Vec3::new(0.0, 0.5, 0.0)),
                    // slide: false,
                    ..default()
                },
                // ActiveEvents::COLLISION_EVENTS,
                // CollisionGroups::new(
                //     Group::GROUP_3,
                //     Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_4,
                // ),
                // AudioBundle {
                //     source: asset_server.load("sounds/footstep.ogg"),
                //     settings: PlaybackSettings::LOOP.with_spatial(true).with_speed(1.0),
                // },
                event.enemy.clone(),
            ))
            .with_children(|parent| {
                parent.spawn((SceneBundle {
                    scene: asset_server.load("models/orc.glb#Scene0"),
                    transform: Transform::from_xyz(0.0, -0.5, 0.0).with_rotation(Quat::from_rotation_y(PI)),
                    ..Default::default()
                },));
            });
    }
}
