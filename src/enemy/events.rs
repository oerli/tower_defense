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
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("models/orc.glb#Scene0"),
                transform: Transform::from_translation(event.position),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
            Velocity {
                linvel: Vec3::new(0.0, 0.0, 0.0),
                angvel: Vec3::new(0.0, 0.0, 0.0),
            },
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(
                Group::GROUP_3,
                Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_4,
            ),
            AudioBundle {
                source: asset_server.load("sounds/footstep.ogg"),
                settings: PlaybackSettings::LOOP.with_spatial(true).with_speed(0.8),
            },
            event.enemy.clone(),
        ));
    }
}
