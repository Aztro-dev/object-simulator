use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::*;
use bevy_flycam::prelude::*;
use bevy_rapier3d::prelude::*;

mod field;
mod light;
mod object;
mod ui;
mod visibility;

use field::FieldPlugin;
use light::spawn_light;
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
                resolution: (1920.0, 1080.0).into(),
                position: WindowPosition::At(IVec2::new(0, 0)),
                decorations: true,
                resizable: true,
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(ObjectPlugin)
        .add_plugin(FieldPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(UiPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00020, // default: 0.00012
            speed: 64.0,          // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::E,
            move_descend: KeyCode::Q,
            ..Default::default()
        })
        .add_startup_system(spawn_light)
        .add_systems((toggle_visibility, close_on_esc))
        .run();
}
