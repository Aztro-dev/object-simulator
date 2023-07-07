use crate::ToggleVisibility;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const GRAVITY: f32 = 2.0;

#[derive(Resource)]
pub struct MyAssetPack(Handle<Gltf>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("F1.glb");
    commands.insert_resource(MyAssetPack(gltf));
}

pub fn spawn_objects(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    my: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // let object_color = Color::hex("FDDA03").unwrap();
    let object_color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    let object_size = 2.0;
    if keyboard_input.pressed(KeyCode::O) {
        if let Some(gltf) = assets_gltf.get(&my.0) {
            commands
                .spawn((
                    RigidBody::Dynamic,
                    TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
                    GravityScale(GRAVITY),
                    Velocity {
                        linvel: Vec3::new(1.0, 1.0, 1.0),
                        angvel: Vec3::new(3.0, 0.0, 0.0),
                    },
                    Collider::cuboid(object_size, 0.7 * object_size, 2.6 * object_size),
                    ColliderDebugColor(object_color.into()),
                    ToggleVisibility {},
                ))
                .insert(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    ..default()
                });
        }
        // .insert(PbrBundle {
        //     mesh: meshes.add(
        //         shape::Cylinder {
        //             radius: object_size,
        //             height: 0.2 * object_size,
        //             resolution: 16,
        //             segments: 16,
        //         }
        //         .into(),
        //     ),
        //     material: materials.add(object_color.into()),
        //     ..default()
        // });
    }
}
