use crate::ToggleVisibility;
use crate::ToggleVisibilityRes;
use bevy::gltf::Gltf;
use bevy::gltf::GltfMesh;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const GRAVITY: f32 = 8.0;

#[derive(Resource)]
pub struct ObjectAssetPack(Handle<Gltf>);

#[derive(Resource)]
pub struct LowPolyPack(Handle<Gltf>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("TriballCustom-V3.glb");
    let low_poly = asset_server.load("TriballLowPoly.glb");
    commands.insert_resource(ObjectAssetPack(gltf));
    commands.insert_resource(LowPolyPack(low_poly));
}

const OBJECT_MODEL_SCALE: f32 = 1.0 / 32.0; // Was 3.0 / 7.0
                                            // const OBJECT_SIZE: f32 = 2.5 / OBJECT_MODEL_SCALE; // Was 3.0
const OBJECT_SPAWN_HEIGHT: f32 = 4.0;
pub fn spawn_objects(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    my: Res<ObjectAssetPack>,
    low_poly: Res<LowPolyPack>,
    toggle_visibility: Res<ToggleVisibilityRes>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    assets: Res<Assets<Mesh>>,
) {
    let object_debug_color = Color::hex("#44b748").unwrap();
    if keyboard_input.just_pressed(KeyCode::O) {
        // if keyboard_input.pressed(KeyCode::O) {
        if let Some(gltf) = assets_gltf.get(&my.0) {
            if let Some(triball_low_poly) =
                assets_gltfmesh.get(&(assets_gltf.get(&low_poly.0).unwrap()).meshes[0])
            {
                commands
                    .spawn(RigidBody::Dynamic)
                    .insert(TransformBundle::from(Transform::from_xyz(
                        0.0,
                        OBJECT_SPAWN_HEIGHT,
                        0.0,
                    )))
                    .insert(GravityScale(GRAVITY))
                    .insert(Velocity {
                        linvel: Vec3::new(10.0, 10.0, 10.0),
                        angvel: Vec3::new(10.0, 0.0, 10.0),
                    })
                    .insert(
                        Collider::from_bevy_mesh(
                            assets.get(&triball_low_poly.primitives[0].mesh).unwrap(),
                            &ComputedColliderShape::TriMesh,
                        )
                        .unwrap(),
                    )
                    .insert(ColliderDebugColor(object_debug_color.into()))
                    .insert(ToggleVisibility {})
                    .insert(SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        visibility: if toggle_visibility.0 {
                            Visibility::Visible
                        } else {
                            Visibility::Hidden
                        },
                        transform: Transform::from_scale(Vec3::splat(OBJECT_MODEL_SCALE)),
                        ..default()
                    });
            }
        }
    }
}
