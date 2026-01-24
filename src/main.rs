use bevy::{log::LogPlugin, prelude::*};
use bevy_skein::SkeinPlugin;
use cables::*;


mod cables;


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
            CablesPlugin,
        ))
        .add_systems(Startup, add_cable)
        .run();
}

fn add_cable(mut commands: Commands) {
    let from = commands.spawn((
        Transform::from_translation(Vec3::ZERO),
        CableConnection { connection_point_offset: -0.5 * Vec3::Y },
    )).id();
    let to = commands.spawn((
        Transform::from_translation(Vec3::Z),
        CableConnection { connection_point_offset: -0.5 * Vec3::Y },
    )).id();
    spawn_cable(&mut commands, &from, &to);
}

