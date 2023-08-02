use crate::maths::*;
use crate::ToggleVisibility;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
// use bevy_third_person_camera::*;

use crate::bevy_third_person_camera::*;

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ThirdPersonCameraPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, handle_movement);
    }
}

#[derive(Component)]
struct Robot {}

const GRAVITY: f32 = 50.0;
const ROBOT_SIZE: f32 = 4.0;
const ROBOT_DEBUG_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            RigidBody::Dynamic,
            GravityScale(GRAVITY),
            ColliderDebugColor(ROBOT_DEBUG_COLOR.into()),
            ToggleVisibility {},
            Collider::cuboid(ROBOT_SIZE, ROBOT_SIZE, ROBOT_SIZE),
            Velocity { ..default() },
            Robot {},
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: ROBOT_SIZE * 2.0,
                })),
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                ..default()
            },
            ThirdPersonCameraTarget,
        ))
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(10.0, -10.0, 0.0),
            ..default()
        }));
}

const ROBOT_SPEED: f32 = 5.0;
const MAX_ROBOT_SPEED: f32 = 50.0;
const ROBOT_TURNING_SPEED: f32 = 1.8;
const MAX_ROBOT_TURNING_SPEED: f32 = 8.0;

fn handle_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut robot_query: Query<(&mut Velocity, &Transform), With<Robot>>,
) {
    for (mut velocity, transform) in robot_query.iter_mut() {
        let curr_angle = quat_to_euler(transform.rotation).y;

        if keyboard_input.pressed(KeyCode::A) && !keyboard_input.pressed(KeyCode::D) {
            velocity.angvel.y += ROBOT_TURNING_SPEED;
            velocity.angvel.y = velocity.angvel.y.min(MAX_ROBOT_TURNING_SPEED);
        } else if keyboard_input.pressed(KeyCode::D) && !keyboard_input.pressed(KeyCode::A) {
            velocity.angvel.y -= ROBOT_TURNING_SPEED;
            velocity.angvel.y = velocity.angvel.y.max(-MAX_ROBOT_TURNING_SPEED);
        }
        if keyboard_input.pressed(KeyCode::W) {
            velocity.linvel.x += ROBOT_SPEED * curr_angle.sin();
            velocity.linvel.z += ROBOT_SPEED * curr_angle.cos();

            velocity.linvel.x = velocity.linvel.x.min(MAX_ROBOT_SPEED);
            velocity.linvel.z = velocity.linvel.z.min(MAX_ROBOT_SPEED);
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity.linvel.x -= ROBOT_SPEED * curr_angle.sin();
            velocity.linvel.z -= ROBOT_SPEED * curr_angle.cos();

            velocity.linvel.x = velocity.linvel.x.max(-MAX_ROBOT_SPEED);
            velocity.linvel.z = velocity.linvel.z.max(-MAX_ROBOT_SPEED);
        }
    }
}
