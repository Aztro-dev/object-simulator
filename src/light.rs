use bevy::prelude::*;

pub fn spawn_light(mut commands: Commands) {
    // Create the lights
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2.0,
    });
    const SPOT_LIGHT_HEIGHT: f32 = 4.0;
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(-5.0, SPOT_LIGHT_HEIGHT, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: 160000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            // inner_angle: 0.6,
            // outer_angle: 0.8,
            range: 100.0,
            radius: 100.0,
            ..default()
        },
        ..default()
    });
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(5.0, SPOT_LIGHT_HEIGHT, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: 160000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            range: 100.0,
            radius: 100.0,
            ..default()
        },
        ..default()
    });
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(0.0, SPOT_LIGHT_HEIGHT, -5.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: 160000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            // inner_angle: 0.6,
            // outer_angle: 0.8,
            range: 100.0,
            radius: 100.0,
            ..default()
        },
        ..default()
    });
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(0.0, SPOT_LIGHT_HEIGHT, 5.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: 160000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            // inner_angle: 0.6,
            // outer_angle: 0.8,
            range: 100.0,
            radius: 100.0,
            ..default()
        },
        ..default()
    });
}
