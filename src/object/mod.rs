use bevy::prelude::*;

mod systems;

use systems::*;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, spawn_objects);
    }
}
