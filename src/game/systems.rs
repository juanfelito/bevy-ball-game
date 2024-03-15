use bevy::prelude::*;

use crate::game::SimulationState;

pub fn pause(
    mut next_simulation_state: ResMut<NextState<SimulationState>>
) {
    next_simulation_state.set(SimulationState::Paused);
}

pub fn resume(
    mut next_simulation_state: ResMut<NextState<SimulationState>>
) {
    next_simulation_state.set(SimulationState::Running);
}

pub fn toggle_simulation (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.eq(&SimulationState::Running) {
            next_simulation_state.set(SimulationState::Paused);
            println!("Paused")
        }

        if simulation_state.eq(&SimulationState::Paused) {
            next_simulation_state.set(SimulationState::Running);
            println!("Running")
        }
    }
}
