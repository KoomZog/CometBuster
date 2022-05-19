use bevy::prelude::*;

#[derive(Component)]
pub struct ChargeLevel(pub f32);
impl Default for ChargeLevel {
    fn default() -> Self {
        Self(0.0)
    }
}