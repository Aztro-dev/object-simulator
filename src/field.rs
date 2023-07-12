use bevy::prelude::*;

// TODO: Actually spawn in the spin up field
pub fn setup_field(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the platform
    let size = 16.0;
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(2.0 * size, 0.125 * size, 2.0 * size).into()),
            transform: Transform::from_xyz(0.0, -size, 0.0),
            material: materials.add(Color::hex("000000").unwrap().into()),
            ..default()
        })
        .insert((
            RigidBody::Fixed,
            Collider::cuboid(size, 0.0625 * size, size),
            ToggleVisibility {},
        ));
}
