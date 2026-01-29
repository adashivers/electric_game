use bevy::{color::palettes::css::BLACK, prelude::*};


pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (load_fonts, spawn_layout).chain());
    }
}

#[derive(Resource)]
struct SparkFont(Handle<Font>);

fn load_fonts(
    mut commands: Commands, asset_server: Res<AssetServer>
) {
    commands.insert_resource(SparkFont(
        asset_server.load::<Font>("fonts/BlockBlueprint.ttf")
    ));
}

fn spawn_layout(
    mut commands: Commands, 
    spark_font: Res<SparkFont>,
) {
    // Text with one section
    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean imperdiet. Etiam ultricies nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus, tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. Nam quam nunc, blandit vel, luctus pulvinar, hendrerit id, lorem. Maecenas nec odio et ante tincidunt tempus. Donec vitae sapien ut libero venenatis faucibus. Nullam quis ante. Etiam sit amet orci eget eros faucibus tincidunt. Duis leo. Sed fringilla mauris sit amet nibh. Donec sodales sagittis magna. Sed consequat, leo eget bibendum sodales, augue velit cursus nunc, "),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: spark_font.0.clone(),
            font_size: 17.0,
            font_smoothing: bevy::text::FontSmoothing::None,
            ..default()
        },
        TextBackgroundColor(BLACK.into()),
        TextShadow::default(),
        // Set the justification of the Text
        TextLayout::new_with_justify(Justify::Left),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            top: percent(0.),
            left: percent(0.),
            right: percent(0.),
            margin: px(50).all(),
            max_width: percent(100.),
            ..default()
        },
    ));
}