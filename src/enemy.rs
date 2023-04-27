use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    bomb::{Bomb, BombResource},
    bullet::{Bullet, BulletResource},
};

#[derive(Component)]
struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Resource)]
struct EnemyResource {
    speed: f32,
    health: f32,
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyResource {
            speed: 200.,
            health: 100.,
        })
        .add_startup_system(Self::spawn_single)
        .add_system(Self::move_horizontal)
        .add_system(Self::wrap_enemy_around_window)
        .add_system(Self::check_bullet_collision)
        .add_system(Self::check_bomb_collision);
    }
}

impl EnemyPlugin {
    fn spawn_single(mut commands: Commands, asset_server: Res<AssetServer>) {
        let enemy_sprite = SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 200., 0.),
                rotation: Quat::from_rotation_z(90_f32.to_radians()),
                ..default()
            },
            texture: asset_server.load("Ships/ship_0009.png"),
            ..default()
        };

        let enemy_bundle = EnemyBundle {
            enemy: Enemy,
            sprite: enemy_sprite,
        };

        commands.spawn(enemy_bundle);
    }

    fn move_horizontal(
        mut enemy_query: Query<&mut Transform, With<Enemy>>,
        enemy_resource: Res<EnemyResource>,
        time: Res<Time>,
    ) {
        for mut enemy_transform in enemy_query.iter_mut() {
            enemy_transform.translation.x -= enemy_resource.speed * time.delta_seconds();
        }
    }

    fn wrap_enemy_around_window(
        mut enemy_query: Query<&mut Transform, With<Enemy>>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        for mut enemy_transform in enemy_query.iter_mut() {
            let window = window_query.single();

            // Calculate the distance from the enemy to the edge of the window
            let distance_to_edge = window.width() / 2. - enemy_transform.translation.x.abs();

            // If the enemy is outside the window, wrap them around to the other side
            if distance_to_edge < 0. {
                // Calculate the offset to move the player by to wrap them around to the other side of the window
                let offset = -enemy_transform.translation.x.signum() * (window.width() / 2. - 1.);
                enemy_transform.translation.x = offset;
            }
        }
    }

    fn check_bullet_collision(
        mut commands: Commands,
        mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
        mut bullet_query: Query<(Entity, &Transform), With<Bullet>>,
        bullet_resource: Res<BulletResource>,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
    ) {
        for (enemy, enemy_transform) in enemy_query.iter_mut() {
            for (bullet, bullet_transform) in bullet_query.iter_mut() {
                if bullet_transform
                    .translation
                    .distance(enemy_transform.translation)
                    <= bullet_resource.radius
                {
                    let bullet_hit_sfx = asset_server.load("Audio/explosionCrunch_000.ogg");
                    audio.play(bullet_hit_sfx);

                    commands.entity(bullet).despawn_recursive();
                    commands.entity(enemy).despawn_recursive();
                }
            }
        }
    }

    fn check_bomb_collision(
        mut commands: Commands,
        mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
        mut bomb_query: Query<(Entity, &Transform), With<Bomb>>,
        bomb_resource: Res<BombResource>,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
    ) {
        for (enemy, enemy_transform) in enemy_query.iter_mut() {
            for (bomb, bomb_transform) in bomb_query.iter_mut() {
                if bomb_transform
                    .translation
                    .distance(enemy_transform.translation)
                    <= bomb_resource.radius
                {
                    let bomb_hit_sfx = asset_server.load("Audio/explosionCrunch_001.ogg");
                    audio.play(bomb_hit_sfx);

                    commands.entity(bomb).despawn_recursive();
                    commands.entity(enemy).despawn_recursive();
                }
            }
        }
    }
}
