use bevy::prelude::*;
use crate::c_movement_and_collisions::*;
use crate::c_tags::*;
use crate::c_shipstats::*;
use crate::c_sprites::*;
use crate::c_chargelevel::*;
use crate::c_lifetime_spawntime::*;
use crate::c_events::*;

#[derive(Bundle)]
pub struct PhysicsObjectBundle {
    pub original: Original,
    pub spawn_sprites: EvCmpSpawnSprites,
    pub global_transform: GlobalTransform,
    pub transform: Transform,
    pub velocity: Velocity,
    pub angle: Angle,
    pub mass: Mass,
    pub radius: Radius,
}
impl Default for PhysicsObjectBundle {
    fn default() -> Self {
        Self {
            original: Original,
            spawn_sprites: EvCmpSpawnSprites,
            global_transform: GlobalTransform::default(),
            transform: Transform::default(),
            velocity: Velocity::default(),
            angle: Angle::default(),
            mass: Mass::default(),
            radius: Radius::default(),
        }
    }
}

#[derive(Bundle)]
pub struct ShipBundle {
    pub player: Player,
    pub collision_type: CollisionType,
    pub sprite_type: SpriteType,
    #[bundle]
    pub physics_object: PhysicsObjectBundle,
    pub ship_stats: ShipStats,
    pub energy: Energy,
    pub charge_level: ChargeLevel,
}
impl Default for ShipBundle {
    fn default() -> Self {
        Self {
            player: Player,
            collision_type: CollisionType::Ship,
            sprite_type: SpriteType::Ship,
            physics_object: PhysicsObjectBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 20.0),
                    ..Default::default()
                },
                mass: Mass(30.0),
                radius: Radius(34.0),
                ..Default::default()
            },
            ship_stats: ShipStats::default(),
            energy: Energy::default(),
            charge_level: ChargeLevel::default(),
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub collision_type: CollisionType,
    pub sprite_type: SpriteType,
    #[bundle]
    pub physics_object: PhysicsObjectBundle,
    pub spawn_time: SpawnTime,
    pub lifetime: Lifetime,
    pub charge_level: ChargeLevel,
}
impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            bullet: Bullet,
            collision_type: CollisionType::Bullet,
            sprite_type: SpriteType::Bullet,
            physics_object: PhysicsObjectBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 10.0),
                    ..Default::default()
                },
                mass: Mass(10.0),
                radius: Radius(4.0),
                ..Default::default()
            },
            spawn_time: SpawnTime(instant::Instant::now()),
            lifetime: Lifetime(instant::Duration::new(1, 0)),
            charge_level: ChargeLevel::default(),
        }
    }
}

#[derive(Bundle)]
pub struct AsteroidBigBundle {
    pub asteroid_size: AsteroidSize,
    pub collision_type: CollisionType,
    pub sprite_type: SpriteType,
    #[bundle]
    pub physics_object: PhysicsObjectBundle,
}
impl Default for AsteroidBigBundle {
    fn default() -> Self {
        Self {
            asteroid_size: AsteroidSize::Big,
            collision_type: CollisionType::Asteroid,
            sprite_type: SpriteType::Asteroid1,
            physics_object: PhysicsObjectBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 10.0),
                    ..Default::default()
                },
                mass: Mass(100.0),
                radius: Radius(88.0),
                ..Default::default()
            }
        }
    }
}

#[derive(Bundle)]
pub struct AsteroidMediumBundle {
    pub asteroid_size: AsteroidSize,
    pub collision_type: CollisionType,
    pub sprite_type: SpriteType,
    #[bundle]
    pub physics_object: PhysicsObjectBundle,
}
impl Default for AsteroidMediumBundle {
    fn default() -> Self {
        Self {
            asteroid_size: AsteroidSize::Medium,
            collision_type: CollisionType::Asteroid,
            sprite_type: SpriteType::Asteroid1,
            physics_object: PhysicsObjectBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 10.0),
                    ..Default::default()
                },
                mass: Mass(50.0),
                radius: Radius(39.0),
                ..Default::default()
            }
        }
    }
}

#[derive(Bundle)]
pub struct AsteroidSmallBundle {
    pub asteroid_size: AsteroidSize,
    pub collision_type: CollisionType,
    pub sprite_type: SpriteType,
    #[bundle]
    pub physics_object: PhysicsObjectBundle,
}
impl Default for AsteroidSmallBundle {
    fn default() -> Self {
        Self {
            asteroid_size: AsteroidSize::Small,
            collision_type: CollisionType::Asteroid,
            sprite_type: SpriteType::Asteroid1,
            physics_object: PhysicsObjectBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 10.0),
                    ..Default::default()
                },
                mass: Mass(20.0),
                radius: Radius(17.0),
                ..Default::default()
            }
        }
    }
}

#[derive(Bundle)]
pub struct ShieldBundle {
    pub shield: Shield,
    pub sprite_type: SpriteType,
    pub spawn_sprites: EvCmpSpawnSprites,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub radius: Radius,
}
impl Default for ShieldBundle {
    fn default() -> Self {
        Self {
            shield: Shield,
            sprite_type: SpriteType::Shield,
            spawn_sprites: EvCmpSpawnSprites,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            radius: Radius(30.0),
        }
    }
}