use std::{collections::VecDeque, str::FromStr};

use bevy::prelude::*;
use bevy_pretty_text::prelude::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<TextQueue>()
        .add_plugins(PrettyTextPlugin)
        .add_systems(Startup, load_ui)
        .add_systems(Update, text_display)
        .add_observer(on_spawn_text)
        .add_observer(on_typewriter_finished);
    }
}

struct TextQueueItem {
    string: String, // text to display when this item gets popped
}

#[derive(Resource, Default)]
pub struct TextQueue{
    queue: VecDeque<TextQueueItem>,
    is_writing: bool,
}

impl TextQueue {
    pub fn push_text(&mut self, text: &str) {
        let Ok(string) = String::from_str(text);
        self.queue.push_back(TextQueueItem { string });
    }
    
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn pop_text(&mut self) -> Option<TextQueueItem> {
        self.queue.pop_front()
    }
}

fn load_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<Glitch>>,
) {
    // load fonts
    let font = asset_server.load::<Font>("fonts/BlockBlueprint.ttf");

    // load styles
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

fn text_display(
    mut commands: Commands, 
    mut text_queue: ResMut<TextQueue>, 
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    text_box: Query<Entity, With<TextBox>>,
) {
    let pressed_advance = keyboard.clear_just_pressed(KeyCode::KeyZ);

    // skip text load
    if pressed_advance {
        match text_box.single() {
            Ok(entity) => {
                if text_queue.is_writing {
                    debug!("skipping text");
                    commands.entity(entity).insert(FinishTypewriter);
                } else {
                    debug!("clearing text");
                    commands.entity(entity).despawn();
                }
            },
            _ => {}
        }
    }
    // display next text
    else if !text_queue.is_writing && !text_queue.is_empty() {
        let do_write = match text_box.single() {
            Ok(entity) => {
                if pressed_advance {
                    // clear previous text
                    commands.entity(entity).despawn();
                }
                pressed_advance
            },
            _ => {
                true
            }
        };

        if do_write {
            // advance queue
            let popped = text_queue.pop_text().unwrap();

            // spawn text
            commands.trigger(SpawnText(popped.string));
            text_queue.is_writing = true;
        }
    }
}

#[derive(Event)]
pub struct SpawnText(pub String);

fn on_typewriter_finished(
    _trigger: On<TypewriterFinished>,
    mut text_queue: ResMut<TextQueue>,
) {
    debug!("typewriter finished");
    text_queue.is_writing = false;
}

#[derive(Component)]
struct TextBox;

fn on_spawn_text(
    trigger: On<SpawnText>,
    mut commands: Commands,
) {
    let parsed_text = PrettyParser::spans(&trigger.0).unwrap();
    // Text with one section
    commands.spawn((
        TextBox,
        Typewriter::new(30.),
        TypewriterIndex::glyph(),
        TextLayout::new_with_justify(Justify::Left),
        parsed_text,
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