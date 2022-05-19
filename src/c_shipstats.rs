use bevy::prelude::*;
use crate::c_controls::Controls;

#[derive(Component)]
pub struct ShipStats {
    pub controls: Controls,
    pub acceleration: f32,
    pub turn_rate: f32,
    pub charge_rate: f32,
    pub bullet_speed: f32,
    pub shield_regeneration: f32,
}
impl Default for ShipStats {
    fn default() -> Self {
        Self {
            controls: Controls::default(),
            acceleration: 300.0,
            turn_rate: 4.0,
            charge_rate: 3.0,
            bullet_speed: 400.0,
            shield_regeneration: 20.0,
        }
    }
}

#[derive(Component)]
pub struct Energy(pub f32);
impl Default for Energy {
    fn default() -> Self {
        Self(100.0)
    }
}