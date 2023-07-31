use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::*;
use bevy_rapier3d::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

mod bevy_third_person_camera;
mod field;
mod light;
mod maths;
mod object;
mod robot;
mod visibility;

use crate::bevy_third_person_camera::ThirdPersonCamera;
use field::FieldPlugin;
use light::spawn_light;
use object::ObjectPlugin;
use robot::RobotPlugin;
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
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            ScreenDiagnosticsPlugin::default(),
            ObjectPlugin,
            RobotPlugin,
            FieldPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
        ))
        .add_systems(Startup, (spawn_light, spawn_camera))
        .add_systems(Update, (toggle_visibility, close_on_esc))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        ThirdPersonCamera {
            zoom_bounds: (20.0, 50.0),
            mouse_sensitivity: 3.5,
            ..default()
        },
    ));
}
