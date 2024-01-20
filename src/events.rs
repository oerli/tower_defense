use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::defense::components::*;
use crate::player::resources::*;

#[derive(Event)]
pub struct BuildEvent {
    button: PointerButton,
    entity: Entity,
}

impl From<ListenerInput<Pointer<Click>>> for BuildEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        BuildEvent {
            button: event.event.button,
            entity: event.target,
        }
    }
}

pub fn build_event(
    mut build_events: EventReader<BuildEvent>,
    transform_query: Query<&GlobalTransform>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<Player>,
) {
    for event in build_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }

        // check if the player has enough money
        if player.score < 10 {
            continue;
        } else {
            player.score -= 10;
        }

        // spawn the defense
        transform_query.get(event.entity).ok().map(|transform| {
            commands
                .spawn((
                    SceneBundle {
                        scene: asset_server.load("models/cannon_tower.glb#Scene0"),
                        transform: transform.clone().into(),
                        ..Default::default()
                    },
                    RigidBody::Dynamic,
                    Defense {
                        targets: VecDeque::new(),
                        damage: 1,
                        shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    },
                    Collider::ball(3.0),
                    Sensor,
                    CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Collider::cuboid(0.5, 0.5, 0.5),
                        CollisionGroups::new(Group::GROUP_2, Group::GROUP_4),
                        PickableBundle::default(),
                    ));
                    parent.spawn((
                        SceneBundle {
                            scene: asset_server.load("models/cannon.glb#Scene0"),
                            transform: Transform::from_xyz(0.0, 0.68, 0.0),
                            ..Default::default()
                        },
                        Weapon,
                    ));
                });
        });

        // disable the build event for multiple clicks
        commands.entity(event.entity).remove::<On<Pointer<Click>>>();
    }
}
