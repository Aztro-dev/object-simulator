use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const GRAVITY: f32 = 0.5;

pub fn spawn_discs(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard_input.just_pressed(KeyCode::A) {
        commands
            .spawn((
                RigidBody::Dynamic,
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
                GravityScale(GRAVITY),
                Velocity {
                    linvel: Vec3::new(1.0, 1.0, 1.0).normalize(),
                    angvel: Vec3::new(0.2, 0.0, 0.0).normalize(),
                },
                Collider::cylinder(0.1, 1.0),
                ColliderDebugColor(Color::hex("FFFF00").unwrap().into()),
            ))
            .insert(PbrBundle {
                mesh: meshes.add(
                    shape::Cylinder {
                        radius: 1.0,
                        height: 0.2,
                        resolution: 16,
                        segments: 64,
                    }
                    .into(),
                ),
                material: materials.add(Color::hex("FFFF00").unwrap().into()),
                ..default()
            });
        // Spawn discs
    }
}
