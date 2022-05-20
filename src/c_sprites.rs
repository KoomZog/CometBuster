use bevy::prelude::*;

pub struct Textures{
    pub ship: Handle<Image>,
    pub shield: Handle<Image>,
    pub bullet: Handle<Image>,
    pub asteroid_1: Handle<Image>,
    pub background: Handle<Image>,
    pub color_gradients: Handle<Image>,
}

#[derive(Component)]
pub enum SpriteType {
    Ship,
    Asteroid1,
    Shield,
    Bullet,
}
impl SpriteType {
    pub fn is_ship(&self) -> bool {
        matches!(*self, SpriteType::Ship)
    }
    pub fn is_shield(&self) -> bool {
        matches!(*self, SpriteType::Shield)
    }
/*
    fn is_asteroid_1(&self) -> bool {
        matches!(*self, SpriteType::Asteroid1)
    }
    fn is_bullet(&self) -> bool {
        matches!(*self, SpriteType::Bullet)
    }
*/
}

#[derive(Clone, Copy, Component)]
pub enum AsteroidSize {
    Small,
    Medium,
    Big
}
impl AsteroidSize {
    pub fn is_small(&self) -> bool {
        matches!(*self, AsteroidSize::Small)
    }
    pub fn is_medium(&self) -> bool {
        matches!(*self, AsteroidSize::Medium)
    }
    pub fn is_big(&self) -> bool {
        matches!(*self, AsteroidSize::Big)
    }
}

