use bevy::prelude::*;

pub mod resources;
pub mod components;
pub mod systems;

use systems::*;
use resources::EnemySpawnTimer;

use crate::AppState;

use super::{confine::systems::confine, player::spawn_player, SimulationState};

pub const SPAWN_GAP: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: u32 = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies.after(spawn_player))
            .add_systems(Update, 
                (
                    enemy_movement.before(confine),
                    update_enemy_direction,
                    enemy_hit_player,
                    spawn_enemies_over_time
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
