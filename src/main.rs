use bevy::prelude::*;
use player::PlayerPlugin;

mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rustaga".into(),
                resolution: (500., 500.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_basic_2d_camera)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn spawn_basic_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
