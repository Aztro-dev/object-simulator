use crate::ToggleVisibility;
use crate::ToggleVisibilityRes;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((spawn_field, spawn_hitboxes));
    }
}

const FIELD_HEIGHT: f32 = -16.0;
const FIELD_SIZE: f32 = 180.0;
const FIELD_THICKNESS: f32 = 0.0;

pub fn spawn_field(
    mut commands: Commands,
    toggle_visibility: Res<ToggleVisibilityRes>,
    asset_server: Res<AssetServer>,
) {
    let field = asset_server.load("JustTheField-V2.glb#Scene0");

    commands
        .spawn((
            RigidBody::Fixed,
            TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
            Collider::cuboid(FIELD_SIZE, FIELD_THICKNESS, FIELD_SIZE),
            ColliderDebugColor(FIELD_DEBUG_COLOR.into()),
            ToggleVisibility {},
        ))
        .insert(SceneBundle {
            scene: field,
            visibility: if toggle_visibility.0 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
            transform: Transform {
                translation: Vec3::new(0.0, FIELD_HEIGHT, 0.0),
                scale: Vec3::splat(3.0 / 7.0),
                ..default()
            },
            ..default()
        });
}

fn wall_hitbox(
    x: f32,
    z: f32,
    flip: bool,
) -> (RigidBody, TransformBundle, Collider, ColliderDebugColor) {
    return (
        RigidBody::Fixed,
        TransformBundle::from(Transform {
            translation: Vec3::new(
                x * FIELD_SIZE / 2.35,
                FIELD_HEIGHT + WALL_SIZE,
                z * FIELD_SIZE / 2.35,
            ),
            rotation: if flip {
                Quat::from_rotation_y(PI / 2.0)
            } else {
                Quat::default()
            },
            ..default()
        }),
        Collider::cuboid(0.0, WALL_SIZE, FIELD_SIZE / 2.3),
        ColliderDebugColor(FIELD_DEBUG_COLOR.into()),
    );
}

const BARRIER_HEIGHT: f32 = 0.5;
const BARRIER_SIZE: f32 = 50.0;
const BARRIER_RADIUS: f32 = 1.2;
fn barrier_hitbox() -> (RigidBody, TransformBundle, Collider, ColliderDebugColor) {
    return (
        RigidBody::Fixed,
        TransformBundle::from(Transform {
            translation: Vec3::new(0.0, FIELD_HEIGHT + BARRIER_RADIUS + BARRIER_HEIGHT, 0.0),
            rotation: Quat::from_rotation_x(PI / 2.0),
            ..default()
        }),
        Collider::cylinder(BARRIER_SIZE, BARRIER_RADIUS),
        ColliderDebugColor(FIELD_DEBUG_COLOR.into()),
    );
}

const WALL_SIZE: f32 = 6.0;
const FIELD_DEBUG_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
pub fn spawn_hitboxes(mut commands: Commands) {
    // Walls
    commands.spawn(wall_hitbox(1.0, 0.0, false));
    commands.spawn(wall_hitbox(-1.0, 0.0, false));
    commands.spawn(wall_hitbox(0.0, 1.0, true));
    commands.spawn(wall_hitbox(0.0, -1.0, true));
    // Barriers
    commands.spawn(barrier_hitbox());
}
