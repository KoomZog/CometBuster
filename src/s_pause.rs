use bevy::prelude::*;
use crate::c_appstate::AppState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, pause.run_if(in_state(AppState::InGame)))
        .add_systems(Update, pause.run_if(in_state(AppState::Paused)))
        ;
    }
}

fn pause (
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if matches!(state.get(), AppState::Paused) {
            next_state.set(AppState::InGame);
        }
        if matches!(state.get(), AppState::InGame) {
            next_state.set(AppState::Paused);
        }
        keyboard_input.clear();
    }
}