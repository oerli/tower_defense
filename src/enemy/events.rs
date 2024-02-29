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
                RigidBody::Dynamic,
                Collider::cuboid(0.25, 0.5, 0.25),
                TransformBundle::from(Transform::from_translation(event.position)),
                VisibilityBundle::default(),
                Velocity::default(),
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(
                    Group::GROUP_3,
                    Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_4,
                ),
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
