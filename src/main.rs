// TO DO - Rough order
// GUI
// Levels

// Prints Rust error messages to the browser console
extern crate console_error_panic_hook;
use std::panic;

extern crate instant; // Works exactly like the std::time counterpart on native, but uses JS performance.now() for WASM

use bevy::{prelude::*};
use rand::Rng;

const SHIP_SPRITE: &str = "ship.png";
const SHIELD_SPRITE: &str = "shield.png";
const BACKGROUND_SPRITE: &str = "background.png";
const ASTEROID_1_SPRITE: &str = "asteroid_1.png";
const BULLET_SPRITE: &str = "laser_sprites/01.png";
const PI: f32 = std::f32::consts::PI;
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

struct Materials {
    ship: Handle<ColorMaterial>,
    shield: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    asteroid_1: Handle<ColorMaterial>,
    bullet: Handle<ColorMaterial>,
}

// Tags
struct Original;
struct GridSprite;
struct Player;
struct Shield;
struct Bullet;
struct CameraWorld;

// Events
struct EvSpawnAsteroidFragments{
    transform: Transform,
    velocity: Velocity,
    asteroid_size_destroyed: AsteroidSize
}

// "Event" components
struct EvCmpSpawnSprites;

// With value
struct Lifetime(instant::Duration);
struct SpawnTime(instant::Instant);

struct ScreenShake {
    start_time: instant::Instant,
    duration: instant::Duration,
    amplitude: f32,
}
impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            start_time: instant::Instant::now(),
            duration: instant::Duration::from_secs_f32(0.4),
            amplitude: 4.0,
        }
    }
}

enum CollisionType {
    Ship,
    Asteroid,
    Shield,
    Bullet,
}
impl CollisionType {
    fn is_ship(&self) -> bool {
        matches!(*self, CollisionType::Ship)
    }
    fn is_asteroid(&self) -> bool {
        matches!(*self, CollisionType::Asteroid)
    }
    fn is_shield(&self) -> bool {
        matches!(*self, CollisionType::Shield)
    }
    fn is_bullet(&self) -> bool {
        matches!(*self, CollisionType::Bullet)
    }
}

enum SpriteType {
    Ship,
    Asteroid1,
    Shield,
    Bullet,
}
impl SpriteType {
    fn is_ship(&self) -> bool {
        matches!(*self, SpriteType::Ship)
    }
    fn is_shield(&self) -> bool {
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

#[derive(Clone, Copy)]
enum AsteroidSize {
    Small,
    Medium,
    Big
}
impl AsteroidSize {
    fn is_small(&self) -> bool {
        matches!(*self, AsteroidSize::Small)
    }
    fn is_medium(&self) -> bool {
        matches!(*self, AsteroidSize::Medium)
    }
    fn is_big(&self) -> bool {
        matches!(*self, AsteroidSize::Big)
    }
}

struct Radius(f32);
impl Default for Radius {
    fn default() -> Self {
        Self(20.0)
    }
}

#[derive(Clone, Copy)]
struct Mass(f32);
impl Default for Mass {
    fn default() -> Self {
        Self(100.0)
    }
}

struct Angle(f32);
impl Default for Angle {
    fn default() -> Self {
        Self(PI / 2.0)
    }
}

struct ShipStats {
    acceleration: f32,
    turn_rate: f32,
    charge_rate: f32,
    bullet_speed: f32,
    shield_regeneration: f32,
}
impl Default for ShipStats {
    fn default() -> Self {
        Self {
            acceleration: 300.0,
            turn_rate: 4.0,
            charge_rate: 3.0,
            bullet_speed: 400.0,
            shield_regeneration: 20.0,
        }
    }
}

struct Energy(f32);
impl Default for Energy {
    fn default() -> Self {
        Self(100.0)
    }
}

struct ChargeLevel(f32);
impl Default for ChargeLevel {
    fn default() -> Self {
        Self(0.0)
    }
}

#[derive(Clone, Copy)]
struct Velocity {
    x: f32,
    y: f32,
}
impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

// Bundles

#[derive(Bundle)]
struct PhysicsObjectBundle {
    original: Original,
    spawn_sprites: EvCmpSpawnSprites,
    global_transform: GlobalTransform,
    transform: Transform,
    velocity: Velocity,
    angle: Angle,
    mass: Mass,
    radius: Radius,
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
struct ShipBundle {
    player: Player,
    collision_type: CollisionType,
    sprite_type: SpriteType,
    #[bundle]
    physics_object: PhysicsObjectBundle,
    ship_stats: ShipStats,
    energy: Energy,
    charge_level: ChargeLevel,
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
struct BulletBundle {
    bullet: Bullet,
    collision_type: CollisionType,
    sprite_type: SpriteType,
    #[bundle]
    physics_object: PhysicsObjectBundle,
    spawn_time: SpawnTime,
    lifetime: Lifetime,
    charge_level: ChargeLevel,
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
struct AsteroidBigBundle {
    asteroid_size: AsteroidSize,
    collision_type: CollisionType,
    sprite_type: SpriteType,
    #[bundle]
    physics_object: PhysicsObjectBundle,
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
struct AsteroidMediumBundle {
    asteroid_size: AsteroidSize,
    collision_type: CollisionType,
    sprite_type: SpriteType,
    #[bundle]
    physics_object: PhysicsObjectBundle,
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
struct AsteroidSmallBundle {
    asteroid_size: AsteroidSize,
    collision_type: CollisionType,
    sprite_type: SpriteType,
    #[bundle]
    physics_object: PhysicsObjectBundle,
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
struct ShieldBundle {
    shield: Shield,
    sprite_type: SpriteType,
    spawn_sprites: EvCmpSpawnSprites,
    transform: Transform,
    global_transform: GlobalTransform,
    radius: Radius,
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

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut app = App::new();
//    app.insert_resource(Msaa { samples: 4 }); // TODO: Find out what this does. Was in the web template. Does not seem to be needed.

    // Use webgl2 for the WASM version. Load all Default Plugins except LogPlugin. It needs to be disabled for web.
    #[cfg(target_arch = "wasm32")]
    app.add_plugins_with(DefaultPlugins, |group| {
        group.disable::<bevy::log::LogPlugin>()
    })
    .add_plugin(bevy_webgl2::WebGL2Plugin);

    // Set window options for native, load ALL Default Plugins
    #[cfg(not(target_arch = "wasm32"))]
    app.insert_resource(WindowDescriptor {
        title: "CometBuster".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        cursor_visible: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    app
    .add_startup_system(setup)
    .add_startup_stage(
        "setup_resources",
        SystemStage::single(setup_resources),
    )
    .add_startup_system(spawn_player_and_asteroids)
    .add_system(debug)
    .add_system(despawn_after_lifetime)
    .add_system(drain_energy)
    .add_system(collision_detection)
    .add_system(control)
    .add_system(spawn_sprite_grid)
    .add_system(respawn_player)
    .add_system(spawn_asteroid_fragments)
    .add_system(gain_energy)
    .add_system(movement_translation)
    .add_system(movement_rotation)
    .add_system(edge_looping)
    .add_system(bullet_direction_to_angle)
    .add_system(normalize_angle)
    .add_system(screen_shake)
    ;
    
    app
//    .add_event::<EvRespawnPlayer>()
    .add_event::<EvSpawnAsteroidFragments>()
    ;
    
    app.run();
}

// -- STARTUP SYSTEMS --

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
    .insert(CameraWorld);

    commands.insert_resource(Materials {
        ship: materials.add(asset_server.load(SHIP_SPRITE).into()),
        shield: materials.add(asset_server.load(SHIELD_SPRITE).into()),
        background: materials.add(asset_server.load(BACKGROUND_SPRITE).into()),
        asteroid_1: materials.add(asset_server.load(ASTEROID_1_SPRITE).into()),
        bullet: materials.add(asset_server.load(BULLET_SPRITE).into()),
    });
}

fn setup_resources(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.background.clone(),
        sprite: Sprite::new(Vec2::new(WINDOW_WIDTH + 20.0, WINDOW_HEIGHT + 20.0)),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_player_and_asteroids (
    mut commands: Commands,
){
    let mut positions = Vec::<Vec2>::new();
    
    positions.push(Vec2::default());
    commands.spawn_bundle(ShipBundle {
        ..Default::default()
    })
    .insert(Transform {
        translation: Vec3::new(
            positions.last().unwrap().x,
            positions.last().unwrap().y,
            AsteroidBigBundle::default().physics_object.transform.translation.z,
        ),
        ..Default::default()
    });

    for _i in 0..5 {
        positions.push(random_free_position(&positions));
        commands.spawn_bundle(AsteroidBigBundle {
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(
                positions.last().unwrap().x,
                positions.last().unwrap().y,
                AsteroidBigBundle::default().physics_object.transform.translation.z,
            ),
            ..Default::default()
        })
        .insert(Velocity {
            x: rf32(-100.0, 100.0),
            y: rf32(-100.0, 100.0),
        })
        ;
    }
}

// -- SYSTEMS --

fn debug(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        commands.spawn().insert(ScreenShake::default());
    }
}

fn control(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity,
        &mut Velocity,
        &ShipStats,
        &mut Angle,
        &mut Transform,
        &Energy,
        &mut ChargeLevel,
        With<Player>,
    )>,
    mut query_shield: Query<(Entity, With<Shield>)>,
) {
    if let Ok((entity, mut velocity, ship_stats, mut angle, mut transform, energy, mut charge_level, _)) = query.single_mut() {

        // Activate Shield
        if keyboard_input.just_pressed(KeyCode::Z) && energy.0 > 20. {
            let shield_entity = commands
            .spawn_bundle(ShieldBundle {
                ..Default::default()
            })
            .id();

            commands.entity(entity).push_children(&[shield_entity])
            .insert(CollisionType::Shield);
        }
        // Deactivate Shield
        if keyboard_input.just_released(KeyCode::Z) {
            commands.entity(entity).insert(CollisionType::Ship);
            if let Ok((shield_entity, _)) = query_shield.single_mut() {
                commands.entity(shield_entity).despawn_recursive();
            }
        }

        // Rotation
        if keyboard_input.pressed(KeyCode::Left) {
            angle.0 += ship_stats.turn_rate * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            angle.0 -= ship_stats.turn_rate * time.delta_seconds();
        }

        // Acceleration
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.x += ship_stats.acceleration * angle.0.cos() * time.delta_seconds();
            velocity.y += ship_stats.acceleration * angle.0.sin() * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::R) {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            velocity.x = 0.0;
            velocity.y = 0.0;
        }

        // Fire
        if keyboard_input.pressed(KeyCode::X) {
            charge_level.0 += ship_stats.charge_rate * time.delta_seconds();
            if charge_level.0 > 2.0 {charge_level.0 = 2.0;}
        }
        if keyboard_input.just_released(KeyCode::X) {
            commands.spawn_bundle(BulletBundle {
                ..Default::default()
            })
            .insert(Transform {
                translation: Vec3::new(
                    transform.translation.x + angle.0.cos() * 30.0,
                    transform.translation.y + angle.0.sin() * 30.0,
                    10.0,
                ),
                ..Default::default()
            })
            .insert(Angle(angle.0))
            .insert(Velocity {
                x: velocity.x + angle.0.cos() * ship_stats.bullet_speed,
                y: velocity.y + angle.0.sin() * ship_stats.bullet_speed,
            })
            .insert(ChargeLevel(charge_level.0))
            .insert(Mass(1.0 + charge_level.0))
            ;
            charge_level.0 = ChargeLevel::default().0;
        }
    }
}

fn spawn_sprite_grid (
    mut commands: Commands,
    materials: Res<Materials>,
    mut query: Query<(Entity, &SpriteType, With<EvCmpSpawnSprites>, Option<&AsteroidSize>, Option<&ChargeLevel>)>
){
    for (entity, sprite_type, _ev_cmp_spawn_sprites, asteroid_size, charge_level) in query.iter_mut() {
        commands.entity(entity)
        .remove::<EvCmpSpawnSprites>();

        // -- Z LAYERS --
        // 30 Shield
        // 20 Ship
        // 10 Asteroids, bullets
        // 00 Background

        let sprite_size: f32;
        let material: Handle<ColorMaterial>;
        let z_position: f32;
        if sprite_type.is_ship() {
            material = materials.ship.clone();
            sprite_size = 60.0;
            z_position = 20.0;
        }
        else if let Some(asteroid_size) = asteroid_size {
            material = materials.asteroid_1.clone();
            if asteroid_size.is_big() {
                sprite_size = 180.0;
                z_position = 10.0;
            }
            else if asteroid_size.is_medium() {
                sprite_size = 80.0;
                z_position = 10.0;
            }
            else {
                sprite_size = 36.0;
                z_position = 10.0;
            }
        }
        else if sprite_type.is_shield() {
            material = materials.shield.clone();
            sprite_size = 72.0;
            z_position = 30.0;
        }
        else if let Some(charge_level) = charge_level {
            material = materials.bullet.clone();
            sprite_size = 30.0 * (1.0 + 1.8 * (charge_level.0 / 1.0).floor());
            z_position = 10.0;
        }
        else { // Always initialize values. TODO: Make this sprite something obvious for debugging
            material = materials.ship.clone();
            sprite_size = 200.0;
            z_position = 1000.0;
        }

        let sprite_grid: Vec<Entity> = vec![
            (-1.0 as f32, -1.0 as f32),
            (0.0 as f32, -1.0 as f32),
            (1.0 as f32, -1.0 as f32),
            (-1.0 as f32, 0.0 as f32),
            (0.0 as f32, 0.0 as f32),
            (1.0 as f32, 0.0 as f32),
            (-1.0 as f32, 1.0 as f32),
            (0.0 as f32, 1.0 as f32),
            (1.0 as f32, 1.0 as f32),
            ].into_iter().map(
            |(x_factor, y_factor)|
            commands.spawn_bundle(SpriteBundle {
                material: material.clone(),
                sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
                transform: Transform {
                    translation: Vec3::new(x_factor * WINDOW_WIDTH, y_factor * WINDOW_HEIGHT, z_position),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GridSprite)
            .id()
        ).collect();

        commands.entity(entity).push_children(&sprite_grid);
    }
}

fn respawn_player (
    mut commands: Commands,
    query_free_space: Query<(&Transform, &With<CollisionType>)>,
    mut query_player: Query<With<Player>>,
){
    if let Ok(_) = query_player.single_mut() {
    } else {
        let mut positions = Vec::<Vec2>::new();
        for (transform, _) in query_free_space.iter() {
            positions.push(Vec2::new(
                transform.translation.x,
                transform.translation.y,
            ))
        }
        positions.push(random_free_position(&positions));
        commands.spawn_bundle(ShipBundle {
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(
                positions.last().unwrap().x,
                positions.last().unwrap().y,
                AsteroidBigBundle::default().physics_object.transform.translation.z,
            ),
            ..Default::default()
        })
        ;
    }
}

fn despawn_after_lifetime(
    mut commands: Commands,
    mut query: Query<(Entity, &SpawnTime, &Lifetime)>,
) {
    for (entity, spawn_time, lifetime) in query.iter_mut() {
        if spawn_time.0.elapsed() > lifetime.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn collision_detection (
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Radius, &Transform, &mut Velocity, &Mass, &CollisionType, Option<&AsteroidSize>, Option<&ChargeLevel>)>,
    mut spawn_asteroid_fragments_writer: EventWriter<EvSpawnAsteroidFragments>,
) {
    let mut iter = query.iter_combinations_mut();

    while let Some([
        (entity_1, radius_1, transform_1, mut velocity_1, mass_1, collision_type_1, asteroid_size_1, charge_level_1),
        (entity_2, radius_2, transform_2, mut velocity_2, mass_2, collision_type_2, asteroid_size_2, charge_level_2)
        ]) = iter.fetch_next()
    {
        let distance = shortest_distance(
            transform_1.translation.x,
            transform_1.translation.y,
            transform_2.translation.x,
            transform_2.translation.y,
        );
        if distance < radius_1.0 + radius_2.0 {
            // Player vs Asteroid -> despawn Player
            if collision_type_1.is_ship() && collision_type_2.is_asteroid() {
                commands.entity(entity_1).despawn_recursive();
            }
            if collision_type_1.is_asteroid() && collision_type_2.is_ship() {
                commands.entity(entity_2).despawn_recursive();
            }
            
            // Bullet vs Asteroid -> despawn both
            else if
            collision_type_1.is_asteroid() && collision_type_2.is_bullet() ||
            collision_type_1.is_bullet() && collision_type_2.is_asteroid()
            {
                let asteroid: Entity;
                let asteroid_size: &AsteroidSize;
                let asteroid_transform: &Transform;
                let asteroid_velocity: Velocity;
                let bullet: Entity;
                let charge_level: &ChargeLevel;
                if collision_type_1.is_asteroid() {
                    asteroid = entity_1;
                    asteroid_size = asteroid_size_1.unwrap();
                    asteroid_transform = transform_1;
                    asteroid_velocity = *velocity_1;
                    bullet = entity_2;
                    charge_level = charge_level_2.unwrap();
                } else {
                    asteroid = entity_2;
                    asteroid_size = asteroid_size_2.unwrap();
                    asteroid_transform = transform_2;
                    asteroid_velocity = *velocity_2;
                    bullet = entity_1;
                    charge_level = charge_level_1.unwrap();
                }
                if asteroid_size.is_big() && charge_level.0 >= 2.0 ||
                asteroid_size.is_medium() && charge_level.0 >= 1.0 ||
                asteroid_size.is_small() {
                    commands.entity(bullet).despawn_recursive();
                    commands.entity(asteroid).despawn_recursive();
                    spawn_asteroid_fragments_writer.send(EvSpawnAsteroidFragments{transform: *asteroid_transform, velocity: asteroid_velocity, asteroid_size_destroyed: *asteroid_size});
                } else {
                    collision_bounce(
                        &mut commands,
                        transform_1.translation,
                        &mut velocity_1,
                        mass_1.0,
                        transform_2.translation,
                        &mut velocity_2,
                        mass_2.0,
                        &time,
                    );
                }
            }

            // Asteroid vs Asteroid, Asteroid vs Shield -> Bounce
            else if
            collision_type_1.is_asteroid() && collision_type_2.is_asteroid() ||
            collision_type_1.is_asteroid() && collision_type_2.is_shield() ||
            collision_type_1.is_shield() && collision_type_2.is_asteroid()
            {
                collision_bounce(
                    &mut commands,
                    transform_1.translation,
                    &mut velocity_1,
                    mass_1.0,
                    transform_2.translation,
                    &mut velocity_2,
                    mass_2.0,
                    &time,
                );
            }
        }
    }
}

fn spawn_asteroid_fragments (
    mut commands: Commands,
    mut spawn_asteroid_fragment_reader: EventReader<EvSpawnAsteroidFragments>,
) {
    let added_velocity = 80.0;
    let retained_velocity_factor = 0.9;
    for event in spawn_asteroid_fragment_reader.iter() {
        let start_angle = rf32(0.0, 2.0 * PI / 3.0);
        if event.asteroid_size_destroyed.is_big() {
            for i in 0..3 {
                let j = i as f32;
                let spawn_circle_radius: f32 = AsteroidBigBundle::default().physics_object.radius.0 * 0.54;
                let x_pos = event.transform.translation.x + (j * 2.0 * PI / 3.0 + start_angle).cos() * spawn_circle_radius;
                let y_pos = event.transform.translation.y + (j * 2.0 * PI / 3.0 + start_angle).sin() * spawn_circle_radius;
                let x_vel = rf32(-added_velocity, added_velocity);
                let y_vel = rf32(-added_velocity, added_velocity);
                commands.spawn_bundle(AsteroidMediumBundle::default())
                .insert(Transform {
                    translation: Vec3::new(x_pos, y_pos, event.transform.translation.z),
                    ..Default::default()
                })
                .insert(Velocity{x: event.velocity.x * retained_velocity_factor + x_vel, y: event.velocity.y * retained_velocity_factor + y_vel})
                ;
            }
        }
        else if event.asteroid_size_destroyed.is_medium() {
            for i in 0..3 {
                let j = i as f32;
                let spawn_circle_radius: f32 = AsteroidMediumBundle::default().physics_object.radius.0 * 0.54;
                let x_pos = event.transform.translation.x + (j * 2.0 * PI / 3.0 + start_angle).cos() * spawn_circle_radius;
                let y_pos = event.transform.translation.y + (j * 2.0 * PI / 3.0 + start_angle).sin() * spawn_circle_radius;
                let x_vel = rf32(-added_velocity, added_velocity);
                let y_vel = rf32(-added_velocity, added_velocity);
                commands.spawn_bundle(AsteroidSmallBundle::default())
                .insert(Transform {
                    translation: Vec3::new(x_pos, y_pos, event.transform.translation.z),
                    ..Default::default()
                })
                .insert(Velocity{x: event.velocity.x * retained_velocity_factor + x_vel, y: event.velocity.y * retained_velocity_factor + y_vel})
                ;
            }
        }
    }
}

fn gain_energy(time: Res<Time>, mut query: Query<(&ShipStats, &mut Energy)>) {
    if let Ok((ship_stats, mut energy)) = query.single_mut() {
        energy.0 += ship_stats.shield_regeneration * time.delta_seconds();
        if energy.0 >= 100. {
            energy.0 = 100.;
        }
    }
}

fn drain_energy(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Energy, &CollisionType)>,
    mut query_shield: Query<(Entity, With<Shield>)>,
//    mut shield_deactivated_writer: EventWriter<EvDeactivateShield>,
) {
    if let Ok((entity, mut energy, collision_type)) = query.single_mut(){
        if collision_type.is_shield() {
            energy.0 -= 100. * time.delta_seconds();
            if energy.0 <= 0. {
                commands.entity(entity).insert(CollisionType::Ship);
                if let Ok((shield_entity, _)) = query_shield.single_mut() {
                    commands.entity(shield_entity).despawn_recursive();
                }
            }
        }
    }
}

fn movement_translation(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, With<Original>)>
) {
    for (mut transform, velocity, _) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn movement_rotation(
    mut query_root: Query<(&Angle, &Children, With<Original>)>,
    mut query_children: Query<(&mut Transform, With<GridSprite>)>
) {
    for (angle, children, _) in query_root.iter_mut() {
        for child in children.into_iter() {
            if let Ok((mut transform_child, _)) = query_children.get_mut(*child) {
                transform_child.rotation = Quat::from_rotation_z(angle.0);
            }
        }
    }
}

fn edge_looping(
    mut query: Query<(&mut Transform, With<Original>)>,
) {
    for (mut transform, _) in query.iter_mut() {
        if transform.translation.x < -WINDOW_WIDTH / 2. {
            transform.translation.x += WINDOW_WIDTH;
        }
        if transform.translation.x > WINDOW_WIDTH / 2. {
            transform.translation.x -= WINDOW_WIDTH;
        }
        if transform.translation.y < WINDOW_HEIGHT / 2. {
            transform.translation.y += WINDOW_HEIGHT;
        }
        if transform.translation.y > WINDOW_HEIGHT / 2. {
            transform.translation.y -= WINDOW_HEIGHT;
        }
    }
}

fn bullet_direction_to_angle (
    mut query: Query<(&mut Angle, &Velocity, With<Bullet>)>
) {
    for (mut angle, velocity, _) in query.iter_mut() {
        angle.0 = (velocity.y / velocity.x).atan();
        if velocity.x < 0.0 { angle.0 += PI}
    }
}

fn normalize_angle(mut query: Query<&mut Angle>) {
    for mut angle in query.iter_mut() {
        if angle.0 > 2. * PI || angle.0 < -2. * PI {
            angle.0 = angle.0 % (2. * PI);
        }
        if angle.0 < 0. {
            angle.0 += 2. * PI;
        }
    }
}

fn screen_shake (
    mut commands: Commands,
    query: Query<(Entity, &ScreenShake)>,
    mut query_camera: Query<(&mut Transform, With<CameraWorld>)>,
) {
    for (mut transform, _) in query_camera.iter_mut() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        for (entity, screen_shake) in query.iter() {
            if screen_shake.start_time.elapsed() > screen_shake.duration {
                commands.entity(entity).despawn();
            } else {
                let current_amplitude: f32 = screen_shake.amplitude * (screen_shake.duration.as_secs_f32() - screen_shake.start_time.elapsed().as_secs_f32()) / screen_shake.duration.as_secs_f32();
                transform.translation.x += rf32(-current_amplitude, current_amplitude);
                transform.translation.y += rf32(-current_amplitude, current_amplitude);
            }
        }
    }
}

// -- HELPER FUNCTIONS --

// Returns a random f32 from FIRST_ARGUMENT to SECOND_ARGUMENT, not including SECOND_ARGUMENT
fn rf32(low: f32, high: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen::<f32>() * (high - low) + low;
}

// Returns a random position that is not currently occupied by an entity with a CollsionType component
fn random_free_position(
    position_vec: &Vec<Vec2>
) -> Vec2 {
    let mut position_free = false;
    let mut x_pos: f32 = 0.0;
    let mut y_pos: f32 = 0.0;
    while position_free == false {
        position_free = true;
        x_pos = rf32(0.0, WINDOW_WIDTH);
        y_pos = rf32(0.0, WINDOW_HEIGHT);
        for position in position_vec.iter() {
            if shortest_distance(position.x, position.y, x_pos, y_pos) < 200.0 {
                position_free = false;
            }
        }
    }
    return Vec2::new(x_pos, y_pos);
}

// Returns the shortest distance between entities, taking edge looping into account
fn shortest_distance (x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x_dist_1 = (x1 - x2).abs();
    let x_dist_2 = (x1 - (x2 + WINDOW_WIDTH)).abs();
    let x_dist_3 = (x1 - (x2 - WINDOW_WIDTH)).abs();
    let x_min = f32::min(x_dist_1, f32::min(x_dist_2, x_dist_3));

    let y_dist_1 = (y1 - y2).abs();
    let y_dist_2 = (y1 - (y2 + WINDOW_HEIGHT)).abs();
    let y_dist_3 = (y1 - (y2 - WINDOW_HEIGHT)).abs();
    let y_min = f32::min(y_dist_1, f32::min(y_dist_2, y_dist_3));

    return x_min.hypot(y_min);
}

// Returns the new X and Y velocities of entities after they bounce
fn collision_bounce(
    commands: &mut Commands,
    // Get position, velocity and mass of both entities
    t1: Vec3,
    v1: &mut Velocity,
    m1: f32,
    mut t2: Vec3,
    v2: &mut Velocity,
    m2: f32,
    time: &Res<Time>,
) {
    // Check if the entities are moving towards each other
    if shortest_distance(
        t1.x + time.delta_seconds() * v1.x,
        t1.y + time.delta_seconds() * v1.y,
        t2.x + time.delta_seconds() * v2.x,
        t2.y + time.delta_seconds() * v2.y,
    ) <
        shortest_distance(t1.x, t1.y, t2.x, t2.y)
    {
        if (t2.x + WINDOW_WIDTH - t1.x).abs() < (t2.x - t1.x).abs() { t2.x += WINDOW_WIDTH; }
        if (t2.x - WINDOW_WIDTH - t1.x).abs() < (t2.x - t1.x).abs() { t2.x -= WINDOW_WIDTH; }
        if (t2.y + WINDOW_HEIGHT - t1.y).abs() < (t2.y - t1.y).abs() { t2.y += WINDOW_HEIGHT; }
        if (t2.y - WINDOW_HEIGHT - t1.y).abs() < (t2.y - t1.y).abs() { t2.y -= WINDOW_HEIGHT; }

        let mut th1 = (v1.y / v1.x).atan(); // Theta, ent 1
        if v1.x < 0.0 { th1 += PI; } // .atan() can only calculate an angle, not which direction along that angle
        let mut th2 = (v2.y / v2.x).atan(); // Theta, ent 2
        if v2.x < 0.0 { th2 += PI; } // .atan() can only calculate an angle, not which direction along that angle
        let vt1 = v1.x.hypot(v1.y).abs(); // Velocity Total, ent 1
        let vt2 = v2.x.hypot(v2.y).abs(); // Velocity Total, ent 2
        let mut t12 = ((t2.y-t1.y)/(t2.x-t1.x)).atan(); // Theta between the entities
        if t2.x < t1.x { t12 += PI; } // .atan() can only calculate an angle, not which direction along that angle

        let v1_start = v1.clone();

        // https://en.wikipedia.org/wiki/Elastic_collision - Two-dimensional collision with two moving objects
        v1.x = (vt1 * (th1-t12).cos() * ( m1 - m2 ) + 2.0 * m2 * vt2 * ( th2 - t12 ).cos() ) / ( m1 + m2 ) * t12.cos() + vt1 * ( th1 - t12 ).sin() * ( t12 + PI / 2.0 ).cos();
        v1.y = (vt1 * (th1-t12).cos() * ( m1 - m2 ) + 2.0 * m2 * vt2 * ( th2 - t12 ).cos() ) / ( m1 + m2 ) * t12.sin() + vt1 * ( th1 - t12 ).sin() * ( t12 + PI / 2.0 ).sin();
        v2.x = (vt2 * (th2-t12).cos() * ( m2 - m1 ) + 2.0 * m1 * vt1 * ( th1 - t12 ).cos() ) / ( m2 + m1 ) * t12.cos() + vt2 * ( th2 - t12 ).sin() * ( t12 + PI / 2.0 ).cos();
        v2.y = (vt2 * (th2-t12).cos() * ( m2 - m1 ) + 2.0 * m1 * vt1 * ( th1 - t12 ).cos() ) / ( m2 + m1 ) * t12.sin() + vt2 * ( th2 - t12 ).sin() * ( t12 + PI / 2.0 ).sin();

        let change_of_momentum: f32 = m1 * (v1_start.x - v1.x).hypot(v1_start.y - v1.y);

        commands.spawn().insert(ScreenShake{amplitude: 0.0 + change_of_momentum / 4000.0, ..Default::default()});
    }
}