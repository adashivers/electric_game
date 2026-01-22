use bevy::prelude::*;
use bevy_skein::SkeinPlugin;
use cables::*;


mod cables;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SkeinPlugin::default(),
        ))
        .run();
}