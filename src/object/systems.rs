use crate::ToggleVisibility;
use crate::ToggleVisibilityRes;
use bevy::gltf::Gltf;
use bevy::gltf::GltfMesh;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const GRAVITY: f32 = 8.0;

#[derive(Resource)]
pub struct ObjectAssetPack(Handle<Gltf>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("TriballCustom-V3.glb");
    commands.insert_resource(ObjectAssetPack(gltf));
}

const OBJECT_MODEL_SCALE: f32 = 1.0 / 32.0; // Was 3.0 / 7.0
const OBJECT_SIZE: f32 = 2.5 / OBJECT_MODEL_SCALE; // Was 3.0
const OBJECT_SPAWN_HEIGHT: f32 = 4.0;
pub fn spawn_objects(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    my: Res<ObjectAssetPack>,
    toggle_visibility: Res<ToggleVisibilityRes>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    assets: Res<Assets<Mesh>>,
) {
    let object_debug_color = Color::hex("#44b748").unwrap();
    if keyboard_input.just_pressed(KeyCode::O) {
        // if keyboard_input.pressed(KeyCode::O) {
        if let Some(gltf) = assets_gltf.get(&my.0) {
            if let Some(gltf_mesh) = assets_gltfmesh.get(&gltf.meshes[0]) {
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
                            assets.get(&gltf_mesh.primitives[0].mesh).unwrap(),
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

/*
                        TransformBundle::from(Transform::from_xyz(0.0, OBJECT_SPAWN_HEIGHT, 0.0)),
                        GravityScale(GRAVITY),
                        Velocity {
                            linvel: Vec3::new(1.0, 1.0, 1.0).normalize(),
                            angvel: Vec3::new(0.0, 0.0, 0.0),
                        },
                        Collider::from_bevy_mesh(
                            assets.get(&gltf_mesh.primitives[0].mesh).unwrap(),
                            &ComputedColliderShape::TriMesh,
                        ),
                        ColliderDebugColor(object_debug_color.into()),
                        ToggleVisibility {},
*/
