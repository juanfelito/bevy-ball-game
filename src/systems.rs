use bevy::prelude::*;
use bevy::window::{PrimaryWindow, close_on_esc};

use crate::events::GameOver;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if app_state.ne(&AppState::Game) {
            next_app_state.set(AppState::Game);
            println!("Entered game state")
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if app_state.ne(&AppState::MainMenu) {
            next_app_state.set(AppState::MainMenu);
            println!("Entered main menu state")
        }
    }
}

pub fn handle_game_over(
    mut game_over_ereader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    for event in game_over_ereader.read().into_iter() {
        println!("Your final score is: {}!", event.score);
        next_app_state.set(AppState::GameOver);
    }
}

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (
                handle_game_over,
                close_on_esc,
                transition_to_game_state,
                transition_to_main_menu_state
            ));
    }
}
