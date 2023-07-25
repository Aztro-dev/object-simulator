use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::*;
// use bevy_flycam::prelude::*;
use bevy_rapier3d::prelude::*;

mod field;
mod light;
mod movement;
mod object;
mod robot;
mod ui;
mod visibility;

use field::FieldPlugin;
use light::spawn_light;
use movement::PlayerPlugin;
use object::ObjectPlugin;
use robot::RobotPlugin;
use ui::UiPlugin;
use visibility::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("333333").unwrap()))
        .insert_resource(ToggleVisibilityRes(true))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Object Test".into(),
                // resolution: (1920.0, 1080.0).into(),
                // position: WindowPosition::At(IVec2::new(0, 0)),
                decorations: true,
                resizable: true,
                // mode: WindowMode::Windowed,
                mode: WindowMode::Fullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(PlayerPlugin)
        .insert_resource(movement::MovementSettings {
            sensitivity: 0.00020, // default: 0.00012
            speed: 64.0,          // default: 12.0
        })
        .add_plugin(ObjectPlugin)
        .add_plugin(RobotPlugin)
        .add_plugin(FieldPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(UiPlugin)
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_camera)
        .add_systems((toggle_visibility, close_on_esc))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-100.0, 30.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
