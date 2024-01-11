use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::*;
use crate::enemy::components::*;

use super::components::*;

pub fn setup_level(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_tiles: Query<(Entity, &Transform), With<Tile>>,
    query_material: Query<&Handle<StandardMaterial>>,
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

    for (entity, transform) in query_tiles.iter() {
        for position in waypoints.iter() {
            if transform.translation.x == position.x && transform.translation.z == position.z {
                commands.entity(entity).remove::<On<Pointer<Click>>>();
                if let Ok(material_handle) = query_material.get(entity) {
                    if let Some(material) = materials.get_mut(material_handle) {
                        // Modify the color of the material
                        material.base_color = Color::BLUE;
                    }
                }
            }
        }
    }

    commands.spawn(Level {
        enemies: 10,
        separation_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        waypoints,
    });
}

pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut Level>,
    time: Res<Time>,
) {
    for mut level in query.iter_mut() {
        if level.enemies <= 0 {
            continue;
        }

        level.separation_timer.tick(time.delta());

        if level.separation_timer.finished() {            
            level.enemies -= 1;

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
                    material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
                    transform: Transform::from_xyz(-8.0, 0.0, -8.0),
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
                Enemy {
                    speed: 0.1,
                    health: 10,
                    score: 10,
                    waypoint: 0,
                },
            ));
        }
    }
}
