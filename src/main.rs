use bevy::{
    log::LogPlugin, prelude::*
};
use bevy_polyline::PolylinePlugin;
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
            PolylinePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(-5.0, 0.0, 0.0)).looking_to(Vec3::X, Vec3::Y),
        Camera3d::default(),
    ));

    let from = commands.spawn((
        GlobalTransform::from_translation(Vec3::ZERO),
        CableConnection { connection_point_offset: -0.5 * Vec3::Y },
    )).id();
    let to = commands.spawn((
        GlobalTransform::from_translation(Vec3::new(0.0, 2.0, 3.0)),
        CableConnection { connection_point_offset: -0.5 * Vec3::Y },
    )).id();
    spawn_cable(&mut commands, &from, &to);
}