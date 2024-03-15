use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation (
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.eq(&SimulationState::Running) {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Paused")
        }

        if simulation_state.eq(&SimulationState::Paused) {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Running")
        }
    }
}
