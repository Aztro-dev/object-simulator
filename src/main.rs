use bevy::prelude::*;
use bevy::window::*;
use bevy_flycam::prelude::*;
use bevy_rapier3d::prelude::*;

mod object;
mod ui;
mod visibility;

use object::ObjectPlugin;
use ui::UiPlugin;
use visibility::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("333333").unwrap()))
        .insert_resource(ToggleVisibilityRes(true))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Object Test".into(),
                fit_canvas_to_parent: true,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(ObjectPlugin)
        .add_plugin(UiPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00020, // default: 0.00012
            speed: 16.0,          // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::E,
            move_descend: KeyCode::Q,
            ..Default::default()
        })
        .add_startup_system(create_light)
        .add_startup_system(spawn_field)
        .add_system(toggle_visibility)
        .add_system(close_on_esc)
        .run();
}

pub fn create_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

pub fn spawn_field(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 16.0;
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(2.0 * size, 0.125 * size, 2.0 * size).into()),
            transform: Transform::from_xyz(0.0, -size, 0.0),
            material: materials.add(Color::hex("000000").unwrap().into()),
            ..default()
        })
        .insert((
            RigidBody::Fixed,
            Collider::cuboid(size, 0.0625 * size, size),
            ToggleVisibility {},
        ));
}
