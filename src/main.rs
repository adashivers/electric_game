use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    asset::LoadState, input::mouse::AccumulatedMouseMotion, log::LogPlugin, prelude::*
};
use bevy_skein::SkeinPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use electric_grid::*;
use ui::*;

mod electric_grid;
mod ui;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                filter: "electric_game=debug,wgpu_core=warn,wgpu_hal=warn".into(),
                level: bevy::log::Level::INFO,
                custom_layer: |_| None,
                ..Default::default()
            }),
            SkeinPlugin::default(),
            UIPlugin,
        ))
        .add_plugins(ElectricGridPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_towers, move_camera))
        .run();
}

fn setup(mut commands: Commands) {
    debug!("start setup");
    // spawn camera
    commands.spawn((
        Transform::from_translation(Vec3::new(25.0, 30.0, -100.0)).looking_at(Vec3::new(75.0, 0.0, 0.0), Vec3::Y),
        Camera3d::default(),
    ));

    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));
}

fn spawn_towers(
    mut commands: Commands, 
    tower_scene: If<Res<TowerScene>>,
    mut loaded: Local<bool>, 
    server: Res<AssetServer>,
) {
    if *loaded { return; }
    match server.get_load_state(tower_scene.0.0.id()) {
        Some(LoadState::Loaded) => {
            commands.spawn(TowerSpawner(
                vec![
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(50.0, 10.0, 0.0),
                    Vec3::new(100.0, 20.0, 0.0),
                    Vec3::new(150.0, 15.0, 0.0),
                ]
            ));
            *loaded = true;
        },
        _ => {}
    }
    
}



fn move_camera(accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<&mut Transform, With<Camera>>,
) {
    let mut transform = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * 0.01;
        let delta_pitch = -delta.y * 0.01;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}