use bevy::prelude::*;

#[derive(Component)]
pub struct Lifetime(pub instant::Duration);
#[derive(Component)]
pub struct SpawnTime(pub instant::Instant);