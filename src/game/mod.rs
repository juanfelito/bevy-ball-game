use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod star;
pub mod score;
pub mod confine;
pub mod ui;
mod systems;

use confine::ConfinePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use ui::GameUIPlugin;
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
                StarPlugin,
                GameUIPlugin,
            ))
            .add_systems(OnEnter(AppState::Game), pause)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), resume);
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused
}
