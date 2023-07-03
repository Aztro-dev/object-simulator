use crate::ToggleVisibility;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const GRAVITY: f32 = 2.0;

pub fn spawn_objects(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let object_color = Color::hex("FDDA03").unwrap();
    if keyboard_input.pressed(KeyCode::O) {
        commands
            .spawn((
                RigidBody::Dynamic,
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
                GravityScale(GRAVITY),
                Velocity {
                    linvel: Vec3::new(1.0, 1.0, 1.0),
                    angvel: Vec3::new(3.0, 0.0, 0.0),
                },
                Collider::cylinder(0.1, 1.0),
                ColliderDebugColor(object_color.into()),
                ToggleVisibility {},
            ))
            .insert(PbrBundle {
                mesh: meshes.add(
                    shape::Cylinder {
                        radius: 1.0,
                        height: 0.2,
                        resolution: 16,
                        segments: 16,
                    }
                    .into(),
                ),
                material: materials.add(object_color.into()),
                ..default()
            });
    }
}
