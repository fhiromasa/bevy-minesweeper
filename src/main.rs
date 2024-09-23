use bevy::prelude::*;
use board_plugin::BoardPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    let window = Window {
        title: "bevy minesweeper".to_string(),
        resolution: (700., 800.).into(),
        resizable: false,
        ..default()
    };
    let primary_window = Some(window);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window,
        ..default()
    }));
    app.add_plugins(BoardPlugin);

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_systems(Startup, setup_camera);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
