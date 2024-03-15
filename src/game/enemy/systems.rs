use bevy::{audio::Volume, prelude::*};
use bevy::window::PrimaryWindow;
use rand::{random, Rng};

use crate::game::player::PLAYER_SIZE;
use crate::game::player::components::Player;

use crate::game::score::resources::Score;

use crate::events::GameOver;

use crate::game::confine::components::Confined;

use super::components::*;
use super::resources::*;
use super::{ENEMY_SIZE, NUMBER_OF_ENEMIES, ENEMY_SPEED, SPAWN_GAP};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>
) {
    if let Ok(player_transform) = player_query.get_single() {
        let window = window_query.get_single().unwrap();
    
        for _ in 0..NUMBER_OF_ENEMIES {
            let (x, y) = generate_spawn(player_transform, window);
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random(), random()).normalize()
                },
                Confined {
                    size: ENEMY_SIZE
                }
            ));
        }
    }
}

fn generate_spawn(player_transform: &Transform, window: &Window) -> (f32, f32) {
    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x = rand::thread_rng().gen_range(0.0+half_enemy_size..window.width()-half_enemy_size);
    let y = rand::thread_rng().gen_range(0.0+half_enemy_size..window.height()-half_enemy_size);

    if player_transform.translation.distance(Vec3::new(x, y, 0.0)) < SPAWN_GAP {
        return generate_spawn(player_transform, window)
    }

    (x, y)
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * delta;
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let min_x = 0.0 + half_enemy_size;
    let max_x = window.width() - half_enemy_size;
    let min_y = 0.0 + half_enemy_size;
    let max_y = window.height() - half_enemy_size;

    let mut bounced = false;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;

        if translation.x <= min_x || translation.x >= max_x {
            enemy.direction.x *= -1.0;
            bounced = true
        }

        if translation.y <= min_y || translation.y >= max_y {
            enemy.direction.y *= -1.0;
            bounced = true
        }
    }

    let sound = if random::<f32>() < 0.5 {
        "audio/pluck_001.ogg"
    } else {
        "audio/pluck_002.ogg"
    };

    if bounced {
        commands.spawn(
            AudioBundle {
                source: asset_server.load(sound),
                settings: PlaybackSettings::ONCE.with_volume(Volume::new(0.3)),
            }
        );
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_ewriter: EventWriter<GameOver>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance <= (player_radius + enemy_radius) {
                commands.spawn(
                    AudioBundle {
                        source: asset_server.load("audio/explosionCrunch_000.ogg"),
                        settings: PlaybackSettings::ONCE.with_volume(Volume::new(0.2)),
                    }
                );
                commands.entity(player_entity).despawn();
                game_over_ewriter.send(GameOver {score: score.value});
            }
        }
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
) {
    if let Ok(player_transform) = player_query.get_single() {
        if enemy_spawn_timer.timer.tick(time.delta()).finished() {
            let window = window_query.get_single().unwrap();
            let (x, y) = generate_spawn(player_transform, window);
    
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random(), random()).normalize()
                },
                Confined {
                    size: ENEMY_SIZE
                }
            ));
        }
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}
