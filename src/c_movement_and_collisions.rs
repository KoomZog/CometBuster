use bevy::prelude::*;

#[derive(Component)]
pub struct Radius(pub f32);
impl Default for Radius {
    fn default() -> Self {
        Self(20.0)
    }
}

#[derive(Clone, Copy, Component)]
pub struct Mass(pub f32);
impl Default for Mass {
    fn default() -> Self {
        Self(100.0)
    }
}

#[derive(Component)]
pub struct Angle(pub f32);
impl Default for Angle {
    fn default() -> Self {
        Self(std::f32::consts::PI / 2.0)
    }
}

#[derive(Clone, Copy, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

#[derive(Component)]
pub enum CollisionType {
    Ship,
    Asteroid,
    Shield,
    Bullet,
}
impl CollisionType {
    pub fn is_ship(&self) -> bool {
        matches!(*self, CollisionType::Ship)
    }
    pub fn is_asteroid(&self) -> bool {
        matches!(*self, CollisionType::Asteroid)
    }
    pub fn is_shield(&self) -> bool {
        matches!(*self, CollisionType::Shield)
    }
    pub fn is_bullet(&self) -> bool {
        matches!(*self, CollisionType::Bullet)
    }
}

