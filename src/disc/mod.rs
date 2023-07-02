use bevy::prelude::*;

mod systems;

use systems::*;

pub struct DiscPlugin;

impl Plugin for DiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_discs);
    }
}
