use bevy::prelude::*;
use bevy::audio::Volume;
use bevy::window::PrimaryWindow;

use super::components::Player;
use super::{PLAYER_SIZE, PLAYER_SPEED};

use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;

use crate::game::score::resources::Score;

use crate::game::confine::components::Confined;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
        Confined {
            size: PLAYER_SIZE
        }
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    star_query: Query<(Entity, &Transform), With<Star>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            if distance <= (player_radius + star_radius) {
                score.value += 1;
                commands.spawn(
                    AudioBundle {
                        source: asset_server.load("audio/laserLarge_000.ogg"),
                        settings: PlaybackSettings::ONCE.with_volume(Volume::new(0.3)),
                    }
                );
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) {
    for player_entity in player_query.iter() {
        commands.entity(player_entity).despawn();
    }
}
