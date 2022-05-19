use bevy::prelude::*;
use crate::c_movement_and_collisions::Velocity;
use crate::c_sprites::AsteroidSize;

#[derive(Component)]
pub struct EvSpawnAsteroidFragments{
    pub transform: Transform,
    pub velocity: Velocity,
    pub asteroid_size_destroyed: AsteroidSize
}

#[derive(Component)]
pub struct EvCmpSpawnSprites;