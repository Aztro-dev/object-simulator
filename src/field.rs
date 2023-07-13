use crate::ToggleVisibility;
use crate::ToggleVisibilityRes;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_field.before(spawn_field));
    }
}

#[derive(Resource)]
pub struct FieldAssetPack(Handle<Gltf>);

pub fn setup_field(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("JustTheField-V2.glb");
    commands.insert_resource(FieldAssetPack(gltf));
}

pub fn spawn_field(
    mut commands: Commands,
    my: Res<FieldAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    toggle_visibility: Res<ToggleVisibilityRes>,
) {
    let object_debug_color = Color::hex("#333333").unwrap();
    let object_size = 4.0;
    if let Some(gltf) = assets_gltf.get(&my.0) {
        commands
            .spawn((
                RigidBody::Fixed,
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
                Collider::cuboid(object_size, 0.5, object_size),
                ColliderDebugColor(object_debug_color.into()),
                ToggleVisibility {},
            ))
            .insert(SceneBundle {
                scene: gltf.scenes[0].clone(),
                visibility: if toggle_visibility.0 {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                transform: Transform::from_scale(Vec3::splat(3.0 / 7.0)),
                ..default()
            });
    }
}
