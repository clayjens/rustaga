use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::PlayerResource;

pub struct ShootBombEvent(pub Transform);

#[derive(Component)]
pub struct Bomb;

#[derive(Bundle)]
struct BombBundle {
    bomb: Bomb,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Resource)]
pub struct BombResource {
    /// The distance the bomb travels per second
    pub speed: f32,
    /// The radius of the bomb (used for collision detection)
    pub radius: f32,
}

pub struct BombPlugin;
impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BombResource {
            speed: 200.,
            radius: 50.,
        })
        .add_event::<ShootBombEvent>()
        .add_system(Self::handle_shoot)
        .add_system(Self::move_bomb)
        .add_system(Self::despawn_if_offscreen);
    }
}

impl BombPlugin {
    fn handle_shoot(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
        mut ev_shoot: EventReader<ShootBombEvent>,
        mut player_resource: ResMut<PlayerResource>,
    ) {
        for ev in ev_shoot.iter() {
            let bomb_sfx = asset_server.load("Audio/laserLarge_000.ogg");
            let player_transform = ev.0;

            if player_resource.bombs > 0 {
                audio.play(bomb_sfx);
                player_resource.bombs -= 1;
                commands.spawn(BombBundle {
                    bomb: Bomb,
                    sprite: SpriteBundle {
                        texture: asset_server.load("Tiles/tile_0012.png"),
                        transform: Transform {
                            translation: Vec3::new(
                                player_transform.translation.x,
                                player_transform.translation.y + 20.,
                                0.,
                            ),
                            ..default()
                        },
                        ..default()
                    },
                });
            }

            println!("Bombs left: {}", player_resource.bombs);
        }
    }

    fn move_bomb(
        mut bomb_query: Query<&mut Transform, With<Bomb>>,
        time: Res<Time>,
        bomb_resource: Res<BombResource>,
    ) {
        for mut transform in bomb_query.iter_mut() {
            transform.translation.y += bomb_resource.speed * time.delta_seconds();
        }
    }

    fn despawn_if_offscreen(
        mut commands: Commands,
        mut bomb_query: Query<(Entity, &Transform), With<Bomb>>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        let window = window_query.single();

        for (entity, transform) in bomb_query.iter_mut() {
            if transform.translation.y > window.height() / 2. {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
