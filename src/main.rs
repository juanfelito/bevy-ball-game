use bevy::prelude::*;

pub mod events;
mod game;
mod main_menu;
mod systems;

use main_menu::MainMenuPlugin;
use game::GamePlugin;

use events::EventPlugin;
use systems::SystemPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins((EventPlugin, GamePlugin, MainMenuPlugin, SystemPlugin))
        .run();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
