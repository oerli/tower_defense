use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::defense::components::*;

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for event in build_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }

        // spawn the defense
        transform_query.get(event.entity).ok().map(|transform| {
            commands
                .spawn((
                    SceneBundle {
                        scene: asset_server.load("models/tower.glb#Scene0"),
                        transform: transform.clone().into(),
                        ..Default::default()
                    },
                    // PbrBundle {
                    //     mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
                    //     material: materials.add(Color::rgb(0.3, 0.4, 0.5).into()),
                    //     transform: transform.clone().into(),
                    //     ..Default::default()
                    // },
                    RigidBody::Dynamic,
                    Defense {
                        targets: vec![],
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
                });
        });

        // disable the build event for multiple clicks
        commands.entity(event.entity).remove::<On<Pointer<Click>>>();
    }
}
