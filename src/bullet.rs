use bevy::{prelude::*, window::PrimaryWindow};

pub struct ShootBulletEvent(pub Transform);

#[derive(Component)]
struct Bullet;

#[derive(Bundle)]
struct BulletBundle {
    bullet: Bullet,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Resource)]
struct BulletResource {
    speed: f32,
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletResource { speed: 300. })
            .add_event::<ShootBulletEvent>()
            .add_system(Self::handle_shoot)
            .add_system(Self::move_bullet)
            .add_system(Self::despawn_if_offscreen);
    }
}

impl BulletPlugin {
    pub fn handle_shoot(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
        mut ev_shoot: EventReader<ShootBulletEvent>,
    ) {
        for ev in ev_shoot.iter() {
            let laser_sfx = asset_server.load("Audio/laserSmall_000.ogg");
            audio.play(laser_sfx);
            let player_transform = ev.0;

            commands.spawn(BulletBundle {
                bullet: Bullet,
                sprite: SpriteBundle {
                    texture: asset_server.load("Tiles/tile_0002.png"),
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

    fn move_bullet(
        mut bullet_query: Query<&mut Transform, With<Bullet>>,
        time: Res<Time>,
        bullet_resource: Res<BulletResource>,
    ) {
        for mut transform in bullet_query.iter_mut() {
            transform.translation.y += bullet_resource.speed * time.delta_seconds();
        }
    }

    fn despawn_if_offscreen(
        mut commands: Commands,
        mut query: Query<(Entity, &Transform), With<Bullet>>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        let window = window_query.single();

        for (entity, transform) in query.iter_mut() {
            if transform.translation.y > window.height() / 2. {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
