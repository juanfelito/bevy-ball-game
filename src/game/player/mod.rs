use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;
use crate::AppState;
use crate::game::SimulationState;

use super::confine::systems::confine;
use super::enemy::systems::spawn_enemies;
use super::star::systems::spawn_stars;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), 
                spawn_player
                    .before(spawn_enemies)
                    .before(spawn_stars)
            )
            .add_systems(Update, 
                (
                    player_movement.before(confine),
                    player_hit_star
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
