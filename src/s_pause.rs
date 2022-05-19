use bevy::prelude::*;
use crate::c_appstate::AppState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(pause))
        .add_system_set(SystemSet::on_update(AppState::Paused).with_system(pause))
            ;
    }
}

fn pause (
    mut state: ResMut<State<AppState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if matches!(state.current(), AppState::Paused) {
            state.pop().unwrap();
        }
        if matches!(state.current(), AppState::InGame) {
            state.push(AppState::Paused).unwrap();
        }
        keyboard_input.clear();
    }
}