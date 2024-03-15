use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use super::{STAR_SIZE, NUMBER_OF_STARS};
use super::components::*;
use super::resources::*;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    let half_star_size = STAR_SIZE / 2.0;

    for _ in 0..NUMBER_OF_STARS {
        let x = rand::thread_rng().gen_range(0.0+half_star_size..window.width()-half_star_size);
        let y = rand::thread_rng().gen_range(0.0+half_star_size..window.height()-half_star_size);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {}
        ));
    }
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>
) {
    if star_spawn_timer.timer.tick(time.delta()).finished() {
        let window = window_query.get_single().unwrap();
        let half_star_size = STAR_SIZE / 2.0;

        let x = rand::thread_rng().gen_range(0.0+half_star_size..window.width()-half_star_size);
        let y = rand::thread_rng().gen_range(0.0+half_star_size..window.height()-half_star_size);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {}
        ));
    }
}

pub fn despawn_stars(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>
) {
    for star_entity in star_query.iter() {
        commands.entity(star_entity).despawn();
    }
}
