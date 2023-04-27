use bevy::{prelude::*, window::PrimaryWindow};

pub struct ShootBombEvent(pub Transform);

#[derive(Component)]
struct Bomb;

#[derive(Bundle)]
struct BombBundle {
    bomb: Bomb,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Resource)]
struct BombResource {
    speed: f32,
}

pub struct BombPlugin;
impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BombResource { speed: 200. })
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
    ) {
        for ev in ev_shoot.iter() {
            let bomb_sfx = asset_server.load("Audio/laserLarge_000.ogg");
            audio.play(bomb_sfx);
            let player_transform = ev.0;

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
