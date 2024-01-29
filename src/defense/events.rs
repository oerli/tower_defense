use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;
use super::resources::*;
use crate::enemy::components::*;
use crate::player::resources::*;
use crate::BuildEvent;
use crate::GameState;

pub fn spawn_defense(
    mut build_events: EventReader<BuildEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    defense_selection: Res<DefenseSelection>,
    mut player: ResMut<Player>,
    transform_query: Query<&GlobalTransform>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    query_defense_range: Query<Entity, With<DefenseRange>>,
) {
    for event in build_events.read() {

        // remove all current defense ranges (should only be one)
        for range_entity in query_defense_range.iter() {
            commands.entity(range_entity).despawn_recursive();
        }

        if event.button != PointerButton::Primary {
            continue;
        }

        // workaround for discarding build event when menu is active
        if *game_state.get() != (GameState::Paused) && *game_state.get() != (GameState::Playing) {
            continue;
        }

        // check if the player has enough money
        if player.credits < 10 {
            continue;
        } else {
            player.credits -= 10;
        }

        match defense_selection.selected {
            Weapon::Cannon => {
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
                                damage: 0.5,
                                shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                                targets: VecDeque::new(),
                            },
                            Collider::ball(3.0),
                            Sensor,
                            CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Collider::cuboid(0.5, 0.5, 0.5),
                                CollisionGroups::new(Group::GROUP_2, Group::GROUP_4),
                                PickableBundle::default(),
                                RapierPickable,
                                On::<Pointer<Click>>::send_event::<RangeEvent>(),
                            ));
                            parent.spawn((
                                SceneBundle {
                                    scene: asset_server.load("models/cannon.glb#Scene0"),
                                    transform: Transform::from_xyz(0.0, 0.68, 0.0),
                                    ..Default::default()
                                },
                                Weapon::Cannon,
                            ));
                        });
                });
            }
            Weapon::Ballista => {
                // spawn the defense
                transform_query.get(event.entity).ok().map(|transform| {
                    commands
                        .spawn((
                            SceneBundle {
                                scene: asset_server.load("models/ballista_tower.glb#Scene0"),
                                transform: transform.clone().into(),
                                ..Default::default()
                            },
                            RigidBody::Dynamic,
                            Defense {
                                damage: 0.3,
                                shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                                targets: VecDeque::new(),
                            },
                            Collider::ball(3.0),
                            Sensor,
                            CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Collider::cuboid(0.5, 0.5, 0.5),
                                CollisionGroups::new(Group::GROUP_2, Group::GROUP_4),
                                On::<Pointer<Click>>::send_event::<RangeEvent>(),
                                PickableBundle::default(),
                                RapierPickable,
                            ));
                            parent.spawn((
                                SceneBundle {
                                    scene: asset_server.load("models/ballista.glb#Scene0"),
                                    transform: Transform::from_xyz(0.0, 0.83, 0.0),
                                    ..Default::default()
                                },
                                Weapon::Ballista,
                            ));
                        });
                });
            }
            Weapon::Archer => {
                // spawn the defense
                transform_query.get(event.entity).ok().map(|transform| {
                    commands
                        .spawn((
                            SceneBundle {
                                scene: asset_server.load("models/archer_tower.glb#Scene0"),
                                transform: transform.clone().into(),
                                ..Default::default()
                            },
                            RigidBody::Dynamic,
                            Defense {
                                damage: 0.1,
                                shooting_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                                targets: VecDeque::new(),
                            },
                            Collider::ball(2.0),
                            Sensor,
                            CollisionGroups::new(Group::GROUP_2, Group::GROUP_3),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Collider::cuboid(0.5, 0.5, 0.5),
                                CollisionGroups::new(Group::GROUP_2, Group::GROUP_4),
                                On::<Pointer<Click>>::send_event::<RangeEvent>(),
                                PickableBundle::default(),
                                RapierPickable,
                            ));
                            parent.spawn((Weapon::Archer,));
                        });
                });
            }
        }

        // disable the build event for multiple clicks
        commands.entity(event.entity).remove::<On<Pointer<Click>>>();
        // set to playing state after build
        next_game_state.set(GameState::Playing);
    }
}

#[derive(Event)]
pub struct RangeEvent {
    pub button: PointerButton,
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Click>>> for RangeEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        RangeEvent {
            button: event.event.button,
            entity: event.target,
        }
    }
}

// spawn defense range under the defense entity
pub fn range_event(
    mut range_events: EventReader<RangeEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parent_query: Query<&Parent>,
    defense_range_query: Query<(Entity, &Parent), With<DefenseRange>>,
    collider_query: Query<&Collider>,
) {
    for event in range_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }

        // toggle hide or show for same defense
        let mut show_defense_range = true;

        // despawn all defense ranges, get parent from event and compare parent of defense range entity
        for (range_entity, range_parent) in defense_range_query.iter() {
            if let Ok(event_parent) = parent_query.get(event.entity) {
                if event_parent.get() == range_parent.get() {
                    // defense range was already shown, hide it
                    show_defense_range = false;
                }
            }
            // remove all current defense ranges (should only be one)
            commands.entity(range_entity).despawn_recursive();
        }

        // finished if defense range was already shown
        if show_defense_range == false {
            continue;
        }

        // create defense range info, spawn below defense entity as own entity
        if let Ok(defense_parent) = parent_query.get(event.entity) {
            // get collider range info from parent
            if let Ok(defense_collider) = collider_query.get(defense_parent.get()) {
                // create defense range entity below defense entity
                commands.entity(defense_parent.get()).with_children(|parent| {
                    parent.spawn((
                        PbrBundle {
                            mesh: asset_server.add(Mesh::from(shape::Torus {
                                // subtract ring_radius from range
                                radius: defense_collider.as_ball().unwrap().radius() - 0.02,
                                ring_radius: 0.02,
                                ..Default::default()
                            })),
                            material: asset_server.add(StandardMaterial {
                                base_color: Color::rgb(0.8, 0.2, 0.2),
                                ..Default::default()
                            }),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.3, 0.0)),
                            ..Default::default()
                        },
                        DefenseRange,
                    ));
                });
            }
        }
    }
}

pub fn enemy_contact(
    mut collision_events: EventReader<CollisionEvent>,
    mut defense_query: Query<&mut Defense>,
    enemy_query: Query<&Enemy>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                // add target to defense
                if enemy_query.get(*entity1).is_ok() {  
                    if let Ok(mut defense) = defense_query.get_mut(*entity2) {
                        defense.targets.push_back(*entity1);
                        
                    }
                } else if enemy_query.get(*entity2).is_ok() {
                    if let Ok(mut defense) = defense_query.get_mut(*entity1) {
                        defense.targets.push_back(*entity2);
                    }
                }
            }
            // does not work if enemy is despawned
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                // remove target from defense
                if enemy_query.get(*entity1).is_ok() {  
                    if let Ok(mut defense) = defense_query.get_mut(*entity2) {
                        defense.targets.retain(|&x| x != *entity1);
                    }
                } else if enemy_query.get(*entity2).is_ok() {
                    if let Ok(mut defense) = defense_query.get_mut(*entity1) {
                        defense.targets.retain(|&x| x != *entity2);
                    }
                } 
            }
        }
    }
}
