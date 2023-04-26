use bevy::{prelude::*, window::PrimaryWindow};

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
            .add_system(Self::move_bullet)
            .add_system(Self::despawn_if_offscreen);
    }
}

impl BulletPlugin {
    pub fn spawn_bullet(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        player_transform: &Transform,
    ) {
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
                println!("despawned bullet");
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
