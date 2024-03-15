use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::StarSpawnTimer;
use systems::*;

use crate::AppState;
use crate::game::SimulationState;

pub const NUMBER_OF_STARS: u32 = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StarSpawnTimer>()
            .add_systems(PostStartup, spawn_stars)
            .add_systems(Update, 
                spawn_stars_over_time
                            .run_if(in_state(AppState::Game))
                            .run_if(in_state(SimulationState::Running))
            );
    }
}
