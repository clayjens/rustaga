use bevy::log::LogPlugin;
use bevy::prelude::*;
use bomb::BombPlugin;
use bullet::BulletPlugin;
use player::PlayerPlugin;

mod bomb;
mod bullet;
mod player;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<LogPlugin>()
            .set(WindowPlugin {
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
            }),
    )
    .add_plugin(PlayerPlugin)
    .add_plugin(BulletPlugin)
    .add_plugin(BombPlugin)
    .add_startup_system(spawn_basic_2d_camera)
    .add_system(bevy::window::close_on_esc);

    // bevy_mod_debugdump::print_main_schedule(&mut app);

    app.run();
}

fn spawn_basic_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
