use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use rand::Rng;

use crate::components::*;
use crate::enemy::components::*;
use crate::enemy::events::*;
use crate::resources::*;

use super::components::*;

pub fn setup_level(
    mut commands: Commands,
    query_tiles: Query<(Entity, &Transform), With<Tile>>,
    query_children: Query<&Children>,
    asset_server: Res<AssetServer>,
) {
    let waypoints = vec![
        Vec3::new(-8.0, 0.0, -8.0),
        Vec3::new(-7.0, 0.0, -8.0),
        Vec3::new(-6.0, 0.0, -8.0),
        Vec3::new(-5.0, 0.0, -8.0),
        Vec3::new(-4.0, 0.0, -8.0),
        Vec3::new(-4.0, 0.0, -7.0),
        Vec3::new(-4.0, 0.0, -6.0),
        Vec3::new(-4.0, 0.0, -5.0),
        Vec3::new(-4.0, 0.0, -4.0),
        Vec3::new(-3.0, 0.0, -4.0),
        Vec3::new(-2.0, 0.0, -4.0),
        Vec3::new(-1.0, 0.0, -4.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 0.0, -3.0),
        Vec3::new(0.0, 0.0, -2.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(3.0, 0.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(4.0, 0.0, 1.0),
        Vec3::new(4.0, 0.0, 2.0),
        Vec3::new(4.0, 0.0, 3.0),
        Vec3::new(4.0, 0.0, 4.0),
        Vec3::new(5.0, 0.0, 4.0),
        Vec3::new(6.0, 0.0, 4.0),
        Vec3::new(7.0, 0.0, 4.0),
        Vec3::new(7.0, 0.0, 5.0),
        Vec3::new(7.0, 0.0, 6.0),
        Vec3::new(7.0, 0.0, 7.0),
    ];

    let mut rng = rand::thread_rng();

    for (entity, transform) in query_tiles.iter() {
        for position in waypoints.iter() {
            if transform.translation.x == position.x && transform.translation.z == position.z {
                commands.entity(entity).remove::<On<Pointer<Click>>>();
                commands.entity(entity).remove::<SceneBundle>();

                // remove possible decorations
                match query_children.get(entity) {
                    Ok(children) => {
                        for child in children.iter() {
                            commands.entity(*child).remove::<SceneBundle>();
                        }
                    }
                    Err(_) => {}
                }

                commands
                    .entity(entity)
                    .insert(SceneBundle {
                        scene: asset_server.load("models/path.glb#Scene0"),
                        transform: Transform::from_xyz(position.x, 0.0, position.z),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        // create some dirt on street
                        if 0.3 > rng.gen() {
                            parent.spawn(SceneBundle {
                                scene: asset_server.load("models/dirt.glb#Scene0"),
                                transform: Transform::from_xyz(0.0, 0.2, 0.0),
                                ..Default::default()
                            });
                        }
                    });
            }
        }
    }

    commands.spawn(Level {
        enemies: 5,
        separation_timer: Timer::from_seconds(2.2, TimerMode::Repeating),
        waypoints,
    });

    commands.insert_resource(Animations(vec![
        // running animation
        asset_server.load("models/orc.glb#Animation3"),
        // dying animation
        asset_server.load("models/orc.glb#Animation9"),
        // jumping animation
        asset_server.load("models/orc.glb#Animation5"),
    ]));
}

pub fn spawn_enemies(
    mut query: Query<&mut Level>,
    time: Res<Time>,
    mut event_writer: EventWriter<SpawnEnemyEvent>,
) {
    for mut level in query.iter_mut() {
        if level.enemies <= 0 {
            continue;
        }

        level.separation_timer.tick(time.delta());

        if level.separation_timer.finished() {
            level.enemies -= 1;
            event_writer.send(SpawnEnemyEvent {
                enemy: Enemy {
                    speed: 0.1,
                    health: 3,
                    score: 10,
                    waypoint: 0,
                },
                position: Vec3::new(-8.0, 0.0, -8.0),
            });            
        }
    }
}
