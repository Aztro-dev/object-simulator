use belly::prelude::*;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BellyPlugin).add_startup_system(create_ui);
    }
}

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
           <br/>
           <span>"Press <ESC> to exit the program"</span>
        </body>
    });
}
