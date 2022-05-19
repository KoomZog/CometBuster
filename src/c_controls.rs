use bevy::prelude::*;

pub struct Controls {
    pub accelerate: KeyCode,
    pub turn_left: KeyCode,
    pub turn_right: KeyCode,
    pub fire: KeyCode,
    pub shield: KeyCode,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            accelerate: KeyCode::Up,
            turn_left: KeyCode::Left,
            turn_right: KeyCode::Right,
            fire: KeyCode::X,
            shield: KeyCode::Z,
        }
    }
}