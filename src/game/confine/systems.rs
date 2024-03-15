use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Confined;

pub fn confine(
    mut confined_query: Query<(&mut Transform, &Confined)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, entity) in confined_query.iter_mut() {
        let half_size = entity.size / 2.0;

        let min_x = 0.0 + half_size;
        let max_x = window.width() - half_size;
        let min_y = 0.0 + half_size;
        let max_y = window.height() - half_size;

        let mut translation = transform.translation;

        if translation.x < min_x {
            translation.x = min_x
        } else if translation.x > max_x {
            translation.x = max_x
        }

        if translation.y < min_y {
            translation.y = min_y
        } else if translation.y > max_y {
            translation.y = max_y
        }

        transform.translation = translation;
    }
}
