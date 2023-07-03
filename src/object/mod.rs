use bevy::prelude::*;

mod systems;

use systems::*;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_objects);
    }
}
