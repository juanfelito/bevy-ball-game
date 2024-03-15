use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod star;
pub mod score;
pub mod confine;
mod systems;

use confine::ConfinePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<SimulationState>()
            .add_plugins((
                ConfinePlugin,
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin
            ))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused
}
