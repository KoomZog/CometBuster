// TO DO - Rough order
// Asteroids splitting when destroyed
// Shield bouncing on asteroids, needs mass component
// Levels

// -- Z LAYERS --
// 30 Shield
// 20 Ship
// 10 Asteroids, bullets
// 00 Background

// Prints Rust error messages to the browser console
extern crate console_error_panic_hook;
use std::panic;

extern crate instant; // Works exactly like the std::time counterpart on native, but implements a JS performance.now() for WASM

use bevy::prelude::*;
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
struct ShieldActive;
struct ShieldActivated;
struct ShieldDeactivated;
struct Bullet;
struct Lifetime(instant::Duration);
struct SpawnTime(instant::Instant);
struct Asteroid;
struct AsteroidBig;
//struct AsteroidMedium;
//struct AsteroidSmall;
struct SpawnSprites;

struct Radius(f32);

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

struct Velocity {
    x: f32,
    y: f32,
}
impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut app = App::build();
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

    app.add_startup_system(setup.system())
    .add_startup_stage(
        "setup_resources",
        SystemStage::single(setup_resources.system()),
    )
    .add_system(debug.system())
    .add_system(control.system())
    .add_system(spawn_sprite_grid.system())
    .add_system(respawn_player.system())
    .add_system(respawn_asteroid.system())
    .add_system(despawn_after_lifetime.system())
    .add_system(collision_detection.system())
    .add_system(gain_energy.system())
    .add_system(drain_energy.system())
    .add_system(activate_shield.system())
    .add_system(deactivate_shield.system())
    .add_system(movement_translation.system())
    .add_system(movement_rotation.system())
    .add_system(edge_looping.system())
    .add_system(normalize_angle.system())
    .run();
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

// -- SYSTEMS --

fn debug(
//    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &Transform,
        With<GridSprite>,
    )>,
) {
    if let Ok((transform, _sprite)) = query.single_mut() {
        if keyboard_input.just_pressed(KeyCode::F1) {
            dbg!(transform.translation);
        }
    }
}

fn control(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(
        Entity,
        &mut Velocity,
        &Acceleration,
        &mut Angle,
        &mut Transform,
        &Energy,
        With<Player>,
    )>,
) {
    if let Ok((entity, mut velocity, acceleration, mut angle, mut transform_player, energy, _)) =
        query.single_mut()
    {
        // Shield
        if keyboard_input.just_pressed(KeyCode::Z) && energy.0 > 20. {
            commands.entity(entity).insert(ShieldActivated);
        }
        if keyboard_input.just_released(KeyCode::Z) {
            commands.entity(entity).insert(ShieldDeactivated);
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
                .spawn_bundle(SpriteBundle {
                    material: materials.asteroid_1.clone(),
                    sprite: Sprite::new(Vec2::new(8., 8.)),
                    transform: Transform {
                        translation: Vec3::new(
                            transform_player.translation.x + angle.0.cos() * 30.,
                            transform_player.translation.y + angle.0.sin() * 30.,
                            15.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Velocity {
                    x: velocity.x + angle.0.cos() * bullet_speed,
                    y: velocity.y + angle.0.sin() * bullet_speed,
                })
                .insert(Radius(4.))
                .insert(Bullet)
                .insert(Original)
                .insert(SpawnTime(instant::Instant::now()))
                .insert(Lifetime(instant::Duration::new(1, 0)));
        }
    }
}

fn spawn_sprite_grid (
    mut commands: Commands,
    materials: Res<Materials>,
    mut query: Query<(Entity, Option<&Player>, With<SpawnSprites>)>
){
    for (entity, player, _) in query.iter_mut() {

        commands.entity(entity)
        .remove::<SpawnSprites>();

        let sprite_size: f32;
        let material: Handle<ColorMaterial>;
        let z_position: f32;
        if let Some(_player) = player {
            material = materials.ship.clone();
            sprite_size = 60.0;
            z_position = 20.0;
        } else {
            material = materials.asteroid_1.clone();
            sprite_size = 180.0;
            z_position = 10.0;
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
    mut query: Query<With<Player>>
){
    if let Ok(_) = query.single_mut() {
    } else {
        commands.spawn()
        .insert(Original)
        .insert(Player)
        .insert(GlobalTransform::default())
        .insert(Transform {
            translation: Vec3::new(100.0, 100.0, 10.),
                ..Default::default()
            })
        .insert(Velocity::default())
        .insert(Angle::default())
        .insert(Acceleration::default())
        .insert(Radius(30.0))
        .insert(Energy::default())
        .insert(SpawnSprites);
    }
}

fn respawn_asteroid (
    mut commands: Commands,
    mut query: Query<With<Asteroid>>
){
    if let Ok(_) = query.single_mut() {
    } else {
        commands.spawn()
        .insert(Original)
        .insert(Asteroid)
        .insert(AsteroidBig)
        .insert(GlobalTransform::default())
        .insert(Transform {
            translation: Vec3::new(rf32(0., WINDOW_WIDTH), rf32(0., WINDOW_HEIGHT), 10.),
                ..Default::default()
            })
        .insert(Velocity {
            x: rf32(-30., 30.),
            y: rf32(-30., 30.),
        })
        .insert(Radius(90.))
        .insert(SpawnSprites);
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

fn collision_detection(
    mut commands: Commands,
    mut query_shield: Query<(&GlobalTransform, &Radius, With<Shield>)>,
    mut query_player: Query<(Entity, &Transform, &Radius, With<Player>, Without<ShieldActive>)>,
    mut query_asteroid: Query<(Entity, &Transform, &Radius, With<Asteroid>)>,
    mut query_bullet: Query<(Entity, &Transform, &Radius, With<Bullet>)>,
) {
    for (asteroid_entity, &asteroid_transform, asteroid_radius, _) in query_asteroid.iter_mut() {

        // Asteroid vs Bullet, destroy Asteroid
        for (bullet_entity, &bullet_transform, bullet_radius, _) in query_bullet.iter_mut() {
            let distance_x = (asteroid_transform.translation.x - bullet_transform.translation.x).abs();
            let distance_y = (asteroid_transform.translation.y - bullet_transform.translation.y).abs();
            let distance = distance_x.hypot(distance_y);
            if distance < bullet_radius.0 + asteroid_radius.0 {
                commands.entity(asteroid_entity).despawn_recursive();
                commands.entity(bullet_entity).despawn_recursive();
            }
        }
        
        // Asteroid vs Shield, destroy Asteroid
        if let Ok((&shield_transform, shield_radius, _)) = query_shield.single_mut() {
            let distance_x = (asteroid_transform.translation.x - shield_transform.translation.x).abs();
            let distance_y = (asteroid_transform.translation.y - shield_transform.translation.y).abs();
            let distance = distance_x.hypot(distance_y);
            if distance < shield_radius.0 + asteroid_radius.0 {
                commands.entity(asteroid_entity).despawn_recursive();
            }
        }

        // Asteroid vs Player, destroy Player
        if let Ok((player_entity, &player_transform, player_radius, _, _)) = query_player.single_mut() {
            let distance_x = (asteroid_transform.translation.x - player_transform.translation.x).abs();
            let distance_y = (asteroid_transform.translation.y - player_transform.translation.y).abs();
            let distance = distance_x.hypot(distance_y);
            if distance < player_radius.0 + asteroid_radius.0 {
                commands.entity(player_entity).despawn_recursive();
            }
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
    mut commands: Commands,
    mut query: Query<(Entity, &mut Energy, With<ShieldActive>)>,
) {
    if let Ok((entity, mut energy, _)) = query.single_mut() {
        energy.0 -= 100. * time.delta_seconds();
        if energy.0 <= 0. {
            commands.entity(entity).insert(ShieldDeactivated);
        }
    }
}

fn activate_shield(
    mut commands: Commands,
    materials: Res<Materials>,
    mut query: Query<(Entity, With<ShieldActivated>)>,
) {
    if let Ok((entity, _)) = query.single_mut() {
        let shield_entity = commands
            .spawn_bundle(SpriteBundle {
                material: materials.shield.clone(),
                sprite: Sprite::new(Vec2::new(80., 80.)),
                transform: Transform {
                    translation: Vec3::new(0., 0., 20.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Shield)
            .insert(Radius(40.))
            //.insert(Angle)
            .id();

        commands.entity(entity).push_children(&[shield_entity]);
        commands.entity(entity).insert(ShieldActive);
        commands.entity(entity).remove::<ShieldActivated>();
    }
}

fn deactivate_shield(
    mut commands: Commands,
    mut query_shield: Query<(Entity, With<Shield>)>,
    mut query_shield_deactivated: Query<(Entity, With<ShieldDeactivated>)>,
) {
    if let Ok((entity, _)) = query_shield_deactivated.single_mut() {
        commands.entity(entity).remove::<ShieldDeactivated>();
        commands.entity(entity).remove::<ShieldActive>();
        if let Ok((entity, _)) = query_shield.single_mut() {
            commands.entity(entity).despawn();
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