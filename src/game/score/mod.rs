use bevy::prelude::*;

pub mod resources;
mod systems;

use systems::*;
use resources::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HighScores>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(Update, (
                update_score.run_if(in_state(AppState::Game)),
                update_high_scores,
                high_scores_updated
            ))
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}