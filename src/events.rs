use bevy::prelude::*;

#[derive(Event)]
pub struct GameOver {
    pub score: u32
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameOver>();
    }
}
