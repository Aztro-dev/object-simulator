use belly::prelude::*;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BellyPlugin)
            .add_startup_system(create_ui)
            .add_system(toggle_visibility);
    }
}

#[derive(Component)]
pub struct TutorialText {}

fn create_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: -1,
            ..default()
        },
        ..default()
    });

    commands.add(StyleSheet::load("ui.ess"));
    commands.add(eml! {
        <body>
           <span>"Press O to spawn the objects"</span>
           <br/>
           <span>"Press P to show/hide hitboxes"</span>
        </body>
    });
}

pub fn toggle_visibility(
    keyboard: Res<Input<KeyCode>>,
    text_query: Query<Entity, With<TutorialText>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::P) || keyboard.just_pressed(KeyCode::O) {
        for text in text_query.iter() {
            commands.entity(text).despawn_recursive();
        }
    }
}
