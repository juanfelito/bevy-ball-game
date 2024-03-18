use bevy::prelude::*;

use crate::AppState;

mod components;
mod styles;
mod systems;

use systems::layout::*;
use systems::interactions::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(Update, (
                press_play_button,
                press_quit_button
            ).run_if(in_state(AppState::MainMenu)));
    }
}
