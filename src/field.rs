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

    let field_debug_color = Color::hex("#000000").unwrap();
    commands
        .spawn((
            RigidBody::Fixed,
            TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
            Collider::cuboid(FIELD_SIZE, FIELD_THICKNESS, FIELD_SIZE),
            ColliderDebugColor(field_debug_color.into()),
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

const WALL_SIZE: f32 = 6.0;
pub fn spawn_hitboxes(mut commands: Commands) {
    let field_debug_color = Color::hex("#000000").unwrap();
    // Walls
    commands.spawn((
        RigidBody::Fixed,
        TransformBundle::from(Transform {
            translation: Vec3::new(-FIELD_SIZE / 2.35, FIELD_HEIGHT + WALL_SIZE, 0.0),
            ..default()
        }),
        Collider::cuboid(0.0, WALL_SIZE, FIELD_SIZE / 2.3),
        ColliderDebugColor(field_debug_color.into()),
    ));
    commands.spawn((
        RigidBody::Fixed,
        TransformBundle::from(Transform {
            translation: Vec3::new(FIELD_SIZE / 2.35, FIELD_HEIGHT + WALL_SIZE, 0.0),
            ..default()
        }),
        Collider::cuboid(0.0, WALL_SIZE, FIELD_SIZE / 2.3),
        ColliderDebugColor(field_debug_color.into()),
    ));
    commands.spawn((
        RigidBody::Fixed,
        TransformBundle::from(Transform {
            translation: Vec3::new(0.0, FIELD_HEIGHT + WALL_SIZE, FIELD_SIZE / 2.35),
            rotation: Quat::from_rotation_y(PI / 2.0),
            ..default()
        }),
        Collider::cuboid(0.0, WALL_SIZE, FIELD_SIZE / 2.3),
        ColliderDebugColor(field_debug_color.into()),
    ));
    commands.spawn((
        RigidBody::Fixed,
        TransformBundle::from(Transform {
            translation: Vec3::new(0.0, FIELD_HEIGHT + WALL_SIZE, -FIELD_SIZE / 2.35),
            rotation: Quat::from_rotation_y(PI / 2.0),
            ..default()
        }),
        Collider::cuboid(0.0, WALL_SIZE, FIELD_SIZE / 2.3),
        ColliderDebugColor(field_debug_color.into()),
    ));
}
