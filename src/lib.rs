use bevy::{input::{keyboard::KeyboardInput, ButtonState}, prelude::*};

pub struct DevCommandPalettePlugin;

impl Plugin for DevCommandPalettePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RebuildCommandPalette>()
            .add_systems(Update, open_command_palette)
            .add_systems(Update, rebuild_command_palette)
            .add_systems(Update, update_text)
            .init_resource::<DevCommandPalette>();
    }
}

#[derive(Resource, Default)]
pub struct DevCommandPalette {
    pub open: bool,
    pub input: String,
}

#[derive(Event)]
struct RebuildCommandPalette;

#[derive(Component)]
struct CommandPaletteRoot;

#[derive(Component)]
struct TextInput;

fn open_command_palette(
    mut r_dev_command_palette: ResMut<DevCommandPalette>,
    mut ev_rebuild_command_palette: EventWriter<RebuildCommandPalette>,
    mut keydown: Res<ButtonInput<KeyCode>>,
) {
    //It is same as VSCode command palette
    if keydown.just_pressed(KeyCode::KeyP) && keydown.pressed(KeyCode::ControlLeft) && keydown.pressed(KeyCode::ShiftLeft) {
        r_dev_command_palette.open = !r_dev_command_palette.open;
        ev_rebuild_command_palette.send(RebuildCommandPalette {});
    }
}

fn rebuild_command_palette(
    mut commands: Commands,
    mut ev_rebuild_command_palette: EventReader<RebuildCommandPalette>,
    q_root: Query<Entity, With<CommandPaletteRoot>>,
    r_dev_command_palette: Res<DevCommandPalette>,
) {
    if ev_rebuild_command_palette.is_empty() {
        return;
    }
    ev_rebuild_command_palette.clear();

    if let Ok(root) = q_root.get_single() {
        commands.entity(root).despawn_recursive();
    }

    if !r_dev_command_palette.open {
        return;
    } else {
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    left: Val::Percent(0.0),
                    top: Val::Percent(0.0),
                    ..default()
                },
                ..default()
            })
            .insert(CommandPaletteRoot).with_children(|commands| {
                //flex direction: row
                commands.spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        flex_direction: FlexDirection::Row,
                        display: Display::Flex,
                        left: Val::Percent(10.0),
                        top: Val::Percent(10.0),
                        width: Val::Percent(80.0),
                        min_height: Val::Px(100.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..default()
                }).with_children(|commands| {
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                margin: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                width: Val::Percent(100.0),
                                height: Val::Px(30.0),
                                ..default()
                            },
                            border_color: Color::rgb(0.8, 0.8, 0.8).into(),
                            ..default()
                        }).with_children(|commands| {
                            commands
                                .spawn(TextBundle {
                                    style: Style {
                                        position_type: PositionType::Relative,
                                        ..default()
                                    },
                                    text: Text::from_section(r_dev_command_palette.input.clone(), TextStyle::default()),
                                    ..default()
                                });
                        });  
                });
            });
    }
}

fn update_text(
    mut r_dev_command_palette: ResMut<DevCommandPalette>,
    mut ev_rebuild_command_palette: EventWriter<RebuildCommandPalette>,
    mut ev_keydown: EventReader<KeyboardInput>,
) {
    if ev_keydown.is_empty() {
        return;
    }

    for ev in ev_keydown.read() {
        let KeyboardInput {
            key_code,
            logical_key,
            state,
            window,
        } = ev;

        if *state == ButtonState::Pressed {
            match logical_key {
                bevy::input::keyboard::Key::Character(val) => {
                    r_dev_command_palette.input.push(val.chars().next().unwrap());
                    ev_rebuild_command_palette.send(RebuildCommandPalette);
                },
                bevy::input::keyboard::Key::Backspace => {
                    r_dev_command_palette.input.pop();
                    ev_rebuild_command_palette.send(RebuildCommandPalette);
                },

                bevy::input::keyboard::Key::Enter => {
                    //TODO
                }
                _ => {
                
                }
            }
        }
    }
}