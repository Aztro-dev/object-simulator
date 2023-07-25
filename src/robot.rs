use crate::ToggleVisibility;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(handle_movement);
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
        ))
        .insert(TransformBundle::from(Transform {
            translation: Vec3::new(10.0, -10.0, 0.0),
            ..default()
        }));
}

fn max(num1: f32, num2: f32) -> f32 {
    if num1 > num2 {
        return num1;
    }
    return num2;
}
fn min(num1: f32, num2: f32) -> f32 {
    if num1 < num2 {
        return num1;
    }
    return num2;
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
        let curr_angle = transform.rotation.to_euler(EulerRot::XYZ).1;

        if keyboard_input.pressed(KeyCode::A) && !keyboard_input.pressed(KeyCode::D) {
            velocity.angvel.y += ROBOT_TURNING_SPEED;
            velocity.angvel.y = min(velocity.angvel.y, MAX_ROBOT_TURNING_SPEED);
        } else if keyboard_input.pressed(KeyCode::D) && !keyboard_input.pressed(KeyCode::A) {
            velocity.angvel.y -= ROBOT_TURNING_SPEED;
            velocity.angvel.y = max(velocity.angvel.y, -MAX_ROBOT_TURNING_SPEED);
            println!("{curr_angle}");
        }
        if keyboard_input.pressed(KeyCode::W) {
            velocity.linvel.x += ROBOT_SPEED * curr_angle.sin();
            velocity.linvel.z += ROBOT_SPEED * curr_angle.cos();

            velocity.linvel.x = min(velocity.linvel.x, MAX_ROBOT_SPEED);
            velocity.linvel.z = min(velocity.linvel.z, MAX_ROBOT_SPEED);
        }
        if keyboard_input.pressed(KeyCode::S) {
            velocity.linvel.x -= ROBOT_SPEED * curr_angle.sin();
            velocity.linvel.z -= ROBOT_SPEED * curr_angle.cos();

            velocity.linvel.x = max(velocity.linvel.x, -MAX_ROBOT_SPEED);
            velocity.linvel.z = max(velocity.linvel.z, -MAX_ROBOT_SPEED);
        }
    }
}
