use bevy::prelude::*;

pub fn setup_defense(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
        material: materials.add(Color::rgb(0.3, 0.4, 0.5).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

pub fn defense_shooting (
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (_entity, mut _transform) in query.iter_mut() {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 0.1))),
                material: materials.add(Color::rgb(0.3, 0.4, 0.5).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            });
        }
    }
}