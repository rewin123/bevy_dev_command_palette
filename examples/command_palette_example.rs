use bevy::prelude::*;
use bevy_dev_command_palette::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DevCommandPalettePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}