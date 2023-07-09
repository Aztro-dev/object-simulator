use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ToggleVisibilityRes(pub bool);

impl ToggleVisibilityRes {
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
}

#[derive(Component)]
pub struct ToggleVisibility {}

pub fn toggle_visibility(
    keyboard: Res<Input<KeyCode>>,
    mut pbr_query: Query<&mut Visibility, With<ToggleVisibility>>,
    mut visibility_res: ResMut<ToggleVisibilityRes>,
) {
    if keyboard.just_pressed(KeyCode::P) {
        for mut visibility in pbr_query.iter_mut() {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
        visibility_res.toggle();
    }
}
