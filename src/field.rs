use crate::ToggleVisibility;
use crate::ToggleVisibilityRes;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_field);
    }
}

pub fn spawn_field(
    mut commands: Commands,
    toggle_visibility: Res<ToggleVisibilityRes>,
    asset_server: Res<AssetServer>,
) {
    let field = asset_server.load("JustTheField-V2.glb#Scene0");

    let object_debug_color = Color::hex("#333333").unwrap();
    let object_size = 4.0;
    commands
        .spawn((
            RigidBody::Fixed,
            TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
            Collider::cuboid(object_size, 0.5, object_size),
            ColliderDebugColor(object_debug_color.into()),
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
                translation: Vec3::new(0.0, -16.0, 0.0),
                scale: Vec3::splat(3.0 / 7.0),
                ..default()
            },
            ..default()
        });
}
