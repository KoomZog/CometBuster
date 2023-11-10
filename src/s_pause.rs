use bevy::prelude::*;
use crate::c_appstate::AppState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(pause.in_set(OnUpdate(AppState::InGame)))
        .add_system(pause.in_set(OnUpdate(AppState::Paused)))
        ;
    }
}

fn pause (
    state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if matches!(state.0, AppState::Paused) {
            next_state.set(AppState::InGame);
        }
        if matches!(state.0, AppState::InGame) {
            next_state.set(AppState::Paused);
        }
        keyboard_input.clear();
    }
}