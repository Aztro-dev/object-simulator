use bevy::prelude::*;

mod systems;

use systems::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
