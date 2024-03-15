use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct ConfinePlugin;

impl Plugin for ConfinePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, confine);
    }
}
