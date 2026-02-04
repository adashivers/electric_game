use std::{f32::consts::PI, str::FromStr};

use bevy::{
    camera::ScalingMode,
    asset::LoadState, 
    color::palettes::css::GREEN, 
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig}, 
    log::LogPlugin, 
    prelude::*
};
use bevy_skein::SkeinPlugin;
// use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use electric_grid::*;
use ui::*;

use crate::electric_grid::{cables::Cable, spark_movement::Spark};

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
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        // Here we define size of our overlay
                        font_size: 42.0,
                        // If we want, we can use a custom font
                        font: default(),
                        // We could also disable font smoothing,
                        ..default()
                    },
                    // We can also change color of the overlay
                    text_color: GREEN.into(),
                    // We can also set the refresh interval for the FPS counter
                    refresh_interval: core::time::Duration::from_millis(100),
                    enabled: true,
                    frame_time_graph_config: FrameTimeGraphConfig {
                        enabled: true,
                        // The minimum acceptable fps
                        min_fps: 30.0,
                        // The target fps
                        target_fps: 144.0,
                    },
                },
            },

        ))
        .add_plugins(ElectricGridPlugin)
        //.add_plugins(EguiPlugin::default())
        //.add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_spark, spawn_towers))
        .run();
}

fn setup(mut commands: Commands) {
    debug!("start setup");

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

fn spawn_spark(
    mut commands: Commands, 
    cables: Query<Entity, With<Cable>>, 
    mut text_queue: ResMut<TextQueue>,
    mut used: Local<bool>
) {
    if *used { return }
    // spawn at random first cable
    if let Some(cable_entity) = cables.iter().next() {
        commands.spawn((
            Spark::new(cable_entity, 1.0),
        )).with_child((
            Transform::from_translation(Vec3::new(100.0, 30.0, -100.0)).looking_at(Vec3::Y * 20.0, Vec3::Y),
            Camera3d::default(),
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize,
                scale: 0.15,
                ..OrthographicProjection::default_3d()
            }),
        ));
        text_queue.push_text("[little spark....|0.2| coming from a place of such violence...|0.2| what does that make you?|1| the conditions of your existence are part of the great fabric humans have woven onto the web of the world.|1| yet, unlike the humans of this world...|0.2| your movement has only a single axis of freedom.\n|2| soar through the power lines, through ceramic containers of transmission towers, through substations that will change your nature.|1| sing your little song of spark and three-phased vibration.\n|2|i hope you are the catalyst of change.|0.2|i love you.|1|](spark)");
        text_queue.push_text("test");
        *used = true;
    }
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


/*
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
*/