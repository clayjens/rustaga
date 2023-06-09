use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bomb::BombPlugin;
use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use evade::EvadePlugin;
use player::PlayerPlugin;

mod bomb;
mod bullet;
mod enemy;
mod evade;
mod player;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .build()
            .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin)
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
    .add_plugin(EnemyPlugin)
    .add_plugin(EvadePlugin)
    .add_startup_system(spawn_basic_2d_camera)
    .add_startup_system(play_background_music)
    .add_system(bevy::window::close_on_esc);

    app.run();
}

fn spawn_basic_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn play_background_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music_sfx = asset_server.load("Audio/Hero-Immortal.ogg");
    audio.play_with_settings(
        music_sfx,
        PlaybackSettings {
            repeat: true,
            ..default()
        },
    );
}
