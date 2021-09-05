// TO DO - Rough order
// Asteroids splitting when destroyed
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
const PI: f32 = std::f32::consts::PI;
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

struct Materials {
    ship: Handle<ColorMaterial>,
    shield: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    asteroid_1: Handle<ColorMaterial>,
}

// Tags
struct Original;
struct GridSprite;
struct Player;
struct Shield;
struct Asteroid;

// Events
struct EvDespawnRecursive{entity: Entity}
struct EvShieldActivated{entity: Entity}
struct EvShieldDeactivated{entity: Entity}
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
    fn is_asteroid_1(&self) -> bool {
        matches!(*self, SpriteType::Asteroid1)
    }
    fn is_shield(&self) -> bool {
        matches!(*self, SpriteType::Shield)
    }
    fn is_bullet(&self) -> bool {
        matches!(*self, SpriteType::Bullet)
    }
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

struct Mass(f32);
impl Default for Mass {
    fn default() -> Self {
        Self(100.0)
    }
}

struct Angle(f32);
impl Default for Angle {
    fn default() -> Self {
        Self(PI / 2.)
    }
}

struct Acceleration(f32);
impl Default for Acceleration {
    fn default() -> Self {
        Self(300.)
    }
}

struct Energy(f32);
impl Default for Energy {
    fn default() -> Self {
        Self(100.)
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
    acceleration: Acceleration,
    energy: Energy,
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
            acceleration: Acceleration::default(),
            energy: Energy::default(),
        }
    }
}

#[derive(Bundle)]
struct BulletBundle {
    collision_type: CollisionType,
    sprite_type: SpriteType,
    #[bundle]
    physics_object: PhysicsObjectBundle,
    spawn_time: SpawnTime,
    lifetime: Lifetime,
}
impl Default for BulletBundle {
    fn default() -> Self {
        Self {
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
//        cursor_visible: false,
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
    .add_system(spawn_sprite_grid)
    .add_system(control)
    .add_system(respawn_player)
    .add_system(despawn_after_lifetime)
    .add_system(collision_detection)
    .add_system(gain_energy)
    .add_system(drain_energy)
    .add_system(activate_shield)
    .add_system(deactivate_shield)
    .add_system(movement_translation)
    .add_system(movement_rotation)
    .add_system(edge_looping)
    .add_system(normalize_angle)
    .add_system(despawn_recursive)
    .add_system(spawn_asteroid_fragments)
    ;
    
    app
    .add_event::<EvShieldActivated>()
    .add_event::<EvShieldDeactivated>()
    .add_event::<EvDespawnRecursive>()
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
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(Materials {
        ship: materials.add(asset_server.load(SHIP_SPRITE).into()),
        shield: materials.add(asset_server.load(SHIELD_SPRITE).into()),
        background: materials.add(asset_server.load(BACKGROUND_SPRITE).into()),
        asteroid_1: materials.add(asset_server.load(ASTEROID_1_SPRITE).into()),
    });
}

fn setup_resources(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.background.clone(),
        sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
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
            x: rf32(0.0, 100.0),
            y: rf32(0.0, 100.0),
        })
        ;
    }
}

// -- SYSTEMS --

fn debug(
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
    }
}

fn control(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity,
        &mut Velocity,
        &Acceleration,
        &mut Angle,
        &mut Transform,
        &Energy,
        With<Player>,
    )>,
    mut shield_activated_writer: EventWriter<EvShieldActivated>,
    mut shield_deactivated_writer: EventWriter<EvShieldDeactivated>,
) {
    if let Ok((entity, mut velocity, acceleration, mut angle, mut transform_player, energy, _)) = query.single_mut() {

        // Shield
        if keyboard_input.just_pressed(KeyCode::Z) && energy.0 > 20. {
            shield_activated_writer.send(EvShieldActivated { entity: entity });
        }
        if keyboard_input.just_released(KeyCode::Z) {
            shield_deactivated_writer.send(EvShieldDeactivated { entity: entity });
        }

        // Rotation
        let rotation_speed: f32 = 4.;
        if keyboard_input.pressed(KeyCode::Left) {
            angle.0 += rotation_speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            angle.0 -= rotation_speed * time.delta_seconds();
        }

        // Acceleration
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.x += acceleration.0 * angle.0.cos() * time.delta_seconds();
            velocity.y += acceleration.0 * angle.0.sin() * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::R) {
            transform_player.translation.x = 0.;
            transform_player.translation.y = 0.;
            velocity.x = 0.;
            velocity.y = 0.
        }

        // Fire
        let bullet_speed: f32 = 400.;
        if keyboard_input.just_pressed(KeyCode::X) {
            commands
                .spawn_bundle(BulletBundle {
                    ..Default::default()
                })
                .insert(Transform {
                    translation: Vec3::new(
                        transform_player.translation.x + angle.0.cos() * 30.,
                        transform_player.translation.y + angle.0.sin() * 30.,
                        10.0,
                    ),
                    ..Default::default()
                })
                .insert(Velocity {
                    x: velocity.x + angle.0.cos() * bullet_speed,
                    y: velocity.y + angle.0.sin() * bullet_speed,
                })
            ;
        }
    }
}

fn spawn_sprite_grid (
    mut commands: Commands,
    materials: Res<Materials>,
    mut query: Query<(Entity, &SpriteType, With<EvCmpSpawnSprites>, Option<&AsteroidSize>)>
){
    for (entity, sprite_type, _ev_cmp_spawn_sprites, asteroid_size) in query.iter_mut() {
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
        else if sprite_type.is_bullet() {
            material = materials.asteroid_1.clone();
            sprite_size = 8.0;
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
    mut query: Query<With<Player>>,
    query_free_space: Query<(&Transform, &With<CollisionType>)>,
){
    if let Ok(_) = query.single_mut() {
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
    mut despawn_recursive_writer: EventWriter<EvDespawnRecursive>,
    mut query: Query<(Entity, &SpawnTime, &Lifetime)>,
) {
    for (entity, spawn_time, lifetime) in query.iter_mut() {
        if spawn_time.0.elapsed() > lifetime.0 {
            despawn_recursive_writer.send(EvDespawnRecursive{entity: entity});
        }
    }
}

fn collision_detection (
    mut query: Query<(Entity, &Radius, &Transform, &mut Velocity, &Mass, &CollisionType)>,
    mut despawn_recursive_writer: EventWriter<EvDespawnRecursive>,
) {
    let mut iter = query.iter_combinations_mut();

    while let Some([
        (entity_1, radius_1, transform_1, mut velocity_1, mass_1, collision_type_1),
        (entity_2, radius_2, transform_2, mut velocity_2, mass_2, collision_type_2)
        ]) = iter.fetch_next()
    {
        let distance = shortest_distance(
            transform_1.translation.x,
            transform_1.translation.y,
            transform_2.translation.x,
            transform_2.translation.y,
        );
        if distance < radius_1.0 + radius_2.0 {

            // Despawn entity 1
            if collision_type_1.is_ship() && collision_type_2.is_asteroid() {
                despawn_recursive_writer.send(EvDespawnRecursive{entity: entity_1});
            }

            // Despawn entity 2
            if collision_type_1.is_asteroid() && collision_type_2.is_ship() {
                despawn_recursive_writer.send(EvDespawnRecursive{entity: entity_2});
            }
            
            // Despawn both
            if
            collision_type_1.is_asteroid() && collision_type_2.is_bullet() ||
            collision_type_1.is_bullet() && collision_type_2.is_asteroid()
            {
                despawn_recursive_writer.send(EvDespawnRecursive{entity: entity_1});
                despawn_recursive_writer.send(EvDespawnRecursive{entity: entity_2});
            }

            // Bounce
            if
            collision_type_1.is_asteroid() && collision_type_2.is_asteroid() ||
            collision_type_1.is_asteroid() && collision_type_2.is_shield() ||
            collision_type_1.is_shield() && collision_type_2.is_asteroid()
            {
                let (velocity_1_x_new, velocity_1_y_new, velocity_2_x_new, velocity_2_y_new) = velocity_after_bounce(
                    transform_1.translation.x,
                    transform_1.translation.y,
                    velocity_1.x,
                    velocity_1.y,
                    mass_1.0,
                    transform_2.translation.x,
                    transform_2.translation.y,
                    velocity_2.x,
                    velocity_2.y,
                    mass_2.0,
                );
                velocity_1.x = velocity_1_x_new;
                velocity_1.y = velocity_1_y_new;
                velocity_2.x = velocity_2_x_new;
                velocity_2.y = velocity_2_y_new;
            }
        }
    }
}

fn despawn_recursive (
    mut commands: Commands,
    mut despawn_recursive_reader: EventReader<EvDespawnRecursive>,
    mut spawn_asteroid_fragment_writer: EventWriter<EvSpawnAsteroidFragments>,
    query: Query<(Entity, &Transform, &Velocity, &AsteroidSize)>
) {
    for event in despawn_recursive_reader.iter() {
        commands.entity(event.entity).despawn_recursive();
        if let Ok((_entity, transform, velocity, asteroid_size)) = query.get(event.entity) {
            spawn_asteroid_fragment_writer.send(EvSpawnAsteroidFragments { transform: *transform, velocity: *velocity, asteroid_size_destroyed: *asteroid_size });
        }
    }
}

fn spawn_asteroid_fragments (
    mut commands: Commands,
    mut spawn_asteroid_fragment_reader: EventReader<EvSpawnAsteroidFragments>,
) {
    for event in spawn_asteroid_fragment_reader.iter() {
        if event.asteroid_size_destroyed.is_big() {
            commands.spawn_bundle(AsteroidMediumBundle::default())
            .insert(Transform {
                translation: Vec3::new(event.transform.translation.x, event.transform.translation.y, event.transform.translation.z),
                ..Default::default()
            })
            .insert(Velocity{x: event.velocity.x, y: event.velocity.y})
            ;
        }
        else if event.asteroid_size_destroyed.is_medium() {
            commands.spawn_bundle(AsteroidSmallBundle::default())
            .insert(Transform {
                translation: Vec3::new(event.transform.translation.x, event.transform.translation.y, event.transform.translation.z),
                ..Default::default()
            })
            .insert(Velocity{x: event.velocity.x, y: event.velocity.y})
            ;
        }
    }
}

fn gain_energy(time: Res<Time>, mut query: Query<&mut Energy>) {
    if let Ok(mut energy) = query.single_mut() {
        energy.0 += 20. * time.delta_seconds();
        if energy.0 >= 100. {
            energy.0 = 100.;
        }
    }
}

fn drain_energy(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Energy, &CollisionType)>,
    mut shield_deactivated_writer: EventWriter<EvShieldDeactivated>,
) {
    if let Ok((entity, mut energy, collision_type)) = query.single_mut(){
        if collision_type.is_shield() {
            energy.0 -= 100. * time.delta_seconds();
            if energy.0 <= 0. {
                shield_deactivated_writer.send(EvShieldDeactivated { entity: entity });
            }
        }
    }
}

fn activate_shield(
    mut commands: Commands,
    mut shield_activated_reader: EventReader<EvShieldActivated>,
) {
    let event = shield_activated_reader.iter().next();
    if event.is_some() {
        let entity = event.unwrap().entity;
        let shield_entity = commands
            .spawn_bundle(ShieldBundle {
                ..Default::default()
            })
            .id();

        commands.entity(entity).push_children(&[shield_entity])
        .insert(CollisionType::Shield);
    }
}

fn deactivate_shield(
    mut commands: Commands,
    mut query_shield: Query<(Entity, With<Shield>)>,
    mut shield_deactivated_reader: EventReader<EvShieldDeactivated>,
) {
    let event = shield_deactivated_reader.iter().next();
    if event.is_some() {
        let entity = event.unwrap().entity;
        commands.entity(entity).insert(CollisionType::Ship);
        if let Ok((entity, _)) = query_shield.single_mut() {
            commands.entity(entity).despawn_recursive();
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
fn velocity_after_bounce(
    // Get position, velocity and mass of both entities
    x1: f32,
    y1: f32,
    xv1: f32,
    yv1: f32,
    m1: f32,
    mut x2: f32,
    mut y2: f32,
    xv2: f32,
    yv2: f32,
    m2: f32,
    // Returns new x and y velocities
) -> (f32, f32, f32, f32) {
    if (x2 + WINDOW_WIDTH - x1).abs() < (x2 - x1).abs() { x2 += WINDOW_WIDTH; }
    if (x2 - WINDOW_WIDTH - x1).abs() < (x2 - x1).abs() { x2 -= WINDOW_WIDTH; }
    if (y2 + WINDOW_HEIGHT - y1).abs() < (y2 - y1).abs() { y2 += WINDOW_HEIGHT; }
    if (y2 - WINDOW_HEIGHT - y1).abs() < (y2 - y1).abs() { y2 -= WINDOW_HEIGHT; }

    let mut t1 = (yv1/xv1).atan(); // Theta, ent 1
    if xv1 < 0.0 { t1 += PI; } // .atan() can only calculate an angle, not which direction along that angle
    let mut t2 = (yv2/xv2).atan(); // Theta, ent 2
    if xv2 < 0.0 { t2 += PI; } // .atan() can only calculate an angle, not which direction along that angle
    let v1 = xv1.hypot(yv1).abs(); // Velocity, ent 1
    let v2 = xv2.hypot(yv2).abs(); // Velocity, ent 2
    let mut t12 = ((y2-y1)/(x2-x1)).atan(); // Theta between the entities
    if x2 < x1 { t12 += PI; } // .atan() can only calculate an angle, not which direction along that angle

    // https://en.wikipedia.org/wiki/Elastic_collision - Two-dimensional collision with two moving objects
    let xv1_new = (v1 * (t1-t12).cos() * ( m1 - m2 ) + 2.0 * m2 * v2 * ( t2 - t12 ).cos() ) / ( m1 + m2 ) * t12.cos() + v1 * ( t1 - t12 ).sin() * ( t12 + PI / 2.0 ).cos();
    let yv1_new = (v1 * (t1-t12).cos() * ( m1 - m2 ) + 2.0 * m2 * v2 * ( t2 - t12 ).cos() ) / ( m1 + m2 ) * t12.sin() + v1 * ( t1 - t12 ).sin() * ( t12 + PI / 2.0 ).sin();
    let xv2_new = (v2 * (t2-t12).cos() * ( m2 - m1 ) + 2.0 * m1 * v1 * ( t1 - t12 ).cos() ) / ( m2 + m1 ) * t12.cos() + v2 * ( t2 - t12 ).sin() * ( t12 + PI / 2.0 ).cos();
    let yv2_new = (v2 * (t2-t12).cos() * ( m2 - m1 ) + 2.0 * m1 * v1 * ( t1 - t12 ).cos() ) / ( m2 + m1 ) * t12.sin() + v2 * ( t2 - t12 ).sin() * ( t12 + PI / 2.0 ).sin();

    return (xv1_new, yv1_new, xv2_new, yv2_new);
}