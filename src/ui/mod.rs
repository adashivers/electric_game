use bevy::{color::palettes::css::BLACK, prelude::*};
use bevy_pretty_text::prelude::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(PrettyTextPlugin)
        .add_systems(Startup, load_fonts_and_styles)
        .add_observer(spawn_text);
    }
}

fn load_fonts_and_styles(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<Glitch>>

) {
    let font = asset_server.load::<Font>("fonts/BlockBlueprint.ttf");

    commands.spawn((
        PrettyStyle("spark"),
        effects![
            Fade {
                frequency: 10.0,
                min: 0.7,
                max: 1.0,
                offset: 1.0,
            },
            PrettyTextMaterial(materials.add(Glitch{
                intensity: 0.02,
                frequency: 50.,
                speed: 8.,
                threshold: 0.95,
            })),
        ],
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font,
            font_size: 17.0,
            font_smoothing: bevy::text::FontSmoothing::None,
            ..default()
        },

        TextColor::WHITE,
        
    ));
}

#[derive(Event)]
pub struct SpawnText;

fn spawn_text(
    _trigger: On<SpawnText>,
    mut commands: Commands, 
) {
    // Text with one section
    commands.spawn((
        Typewriter::new(30.),
        TypewriterIndex::glyph(),
        TextLayout::new_with_justify(Justify::Left),
        TextBackgroundColor(BLACK.into()),
        pretty!("[little spark....|0.2| coming from a place of such violence...|0.2| what does that make you?|1| the conditions of your existence are part of the great fabric humans have woven onto the web of the world.|1| yet, unlike the humans of this world...|0.2| your movement has only a single axis of freedom.\n|2| soar through the power lines, through ceramic containers of transmission towers, through substations that will change your nature.|1| sing your little song of spark and three-phased vibration.\n|2|i hope you are the catalyst of change.|0.2|i love you.|1|](spark)"),
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