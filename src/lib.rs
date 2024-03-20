use bevy::prelude::*;

pub struct DevCommandPalettePlugin;

impl Plugin for DevCommandPalettePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RebuildCommandPalette>()
            .add_systems(Update, open_command_palette)
            .add_systems(Update, rebuild_command_palette)
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
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        display: Display::Flex,
                        left: Val::Percent(0.2),
                        right: Val::Percent(0.2),
                        top: Val::Percent(0.2),
                        min_height: Val::Px(100.0),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..default()
                });
            });
    }
}