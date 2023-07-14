use bevy::prelude::*;

const SPOT_LIGHT_HEIGHT: f32 = 96.0;
const RANGE: f32 = 1000.0;
const INTENSITY: f32 = 160000.0;
const RADIUS: f32 = 1000.0;

pub fn spawn_light(mut commands: Commands) {
    // Create the lights
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2.0,
    });
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(-5.0, SPOT_LIGHT_HEIGHT, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: INTENSITY,
            color: Color::WHITE,
            shadows_enabled: true,
            range: RANGE,
            radius: RADIUS,
            ..default()
        },
        ..default()
    });
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(5.0, SPOT_LIGHT_HEIGHT, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: INTENSITY,
            color: Color::WHITE,
            shadows_enabled: true,
            range: RANGE,
            radius: RADIUS,
            ..default()
        },
        ..default()
    });
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(0.0, SPOT_LIGHT_HEIGHT, -5.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: INTENSITY,
            color: Color::WHITE,
            shadows_enabled: true,
            range: RANGE,
            radius: RADIUS,
            ..default()
        },
        ..default()
    });
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(0.0, SPOT_LIGHT_HEIGHT, 5.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: INTENSITY,
            color: Color::WHITE,
            shadows_enabled: true,
            range: RANGE,
            radius: RADIUS,
            ..default()
        },
        ..default()
    });
}
