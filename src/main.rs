use bevy::prelude::*;
use bevy::window::*;
use bevy_flycam::prelude::*;
use bevy_rapier3d::prelude::*;

mod disc;

use disc::DiscPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("333333").unwrap()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Disc Test".into(),
                fit_canvas_to_parent: true,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(DiscPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00005, // default: 0.00012
            speed: 8.0,           // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::E,
            move_descend: KeyCode::Q,
            ..Default::default()
        })
        .add_startup_system(show_tutorial)
        .add_startup_system(create_light)
        .add_system(toggle_visibility)
        .add_system(close_on_esc)
        .run();
}
#[derive(Component)]
pub struct TutorialText {}

pub fn show_tutorial(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: -1,
            ..default()
        },
        ..default()
    });
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Press P to toggle visibility\nPress <ESC> to exit",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    position: UiRect::all(Val::Px(0.0)),
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                }),
                Label,
            ));
        })
        .insert(TutorialText {});
}

pub fn create_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2.0,
    });
}

#[derive(Component)]
pub struct ToggleVisibility {}

pub fn toggle_visibility(
    keyboard: Res<Input<KeyCode>>,
    mut pbr_query: Query<&mut Visibility, With<ToggleVisibility>>,
    text_query: Query<Entity, With<TutorialText>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::P) {
        for text in text_query.iter() {
            commands.entity(text).despawn_recursive();
        }
        for mut visibility in pbr_query.iter_mut() {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
