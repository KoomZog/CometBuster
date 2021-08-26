// TO DO - Rough order
// Bounce function working over edges
// Asteroids splitting when destroyed
// GUI
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
struct Mass(f32);

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

    app.add_startup_system(setup)
    .add_startup_stage(
        "setup_resources",
        SystemStage::single(setup_resources),
    )
    .add_startup_system(spawn_asteroids)
    .add_system(debug)
    .add_system(control)
    .add_system(spawn_sprite_grid)
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
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
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
                .insert(Radius(4.0))
                .insert(Mass(5.0))
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
            translation: Vec3::new(500.0, -300.0, 10.),
                ..Default::default()
            })
        .insert(Velocity::default())
        .insert(Angle::default())
        .insert(Acceleration::default())
        .insert(Mass(30.0))
        .insert(Radius(30.0))
        .insert(Energy::default())
        .insert(SpawnSprites);
    }
}

fn spawn_asteroids (
    mut commands: Commands,
){
    commands.spawn()
    .insert(Original)
    .insert(Asteroid)
    .insert(AsteroidBig)
    .insert(GlobalTransform::default())
    .insert(Transform {
        translation: Vec3::new(100.0, 100.0, 10.),
            ..Default::default()
        })
    .insert(Velocity {
        x: 100.0,
        y: 0.0,
    })
    .insert(Radius(90.))
    .insert(Mass(100.))
    .insert(SpawnSprites);

    commands.spawn()
    .insert(Original)
    .insert(Asteroid)
    .insert(AsteroidBig)
    .insert(GlobalTransform::default())
    .insert(Transform {
        translation: Vec3::new(800.0, 150.0, 10.),
            ..Default::default()
        })
    .insert(Velocity {
        x: -100.0,
        y: 0.0,
    })
    .insert(Radius(90.))
    .insert(Mass(100.))
    .insert(SpawnSprites);
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
    mut query: Query<(Entity, &Radius, &Transform, &mut Velocity, &Mass, Option<&Player>, Option<&Bullet>, Option<&Asteroid>, Option<&ShieldActive>)>,
) {
    let mut iter = query.iter_combinations_mut();

    while let Some([
        (entity_1, radius_1, transform_1, mut velocity_1, mass_1, player_1, bullet_1, asteroid_1, shield_active_1),
        (entity_2, radius_2, transform_2, mut velocity_2, mass_2, player_2, bullet_2, asteroid_2, shield_active_2)
        ]) = iter.fetch_next()
    {
        let distance = shortest_distance(
            transform_1.translation.x,
            transform_1.translation.y,
            transform_2.translation.x,
            transform_2.translation.y,
        );
        if distance < radius_1.0 + radius_2.0 {

            // Despawn 1
            if
                player_1.is_some() && shield_active_1.is_none() && asteroid_2.is_some() ||
                asteroid_1.is_some() && bullet_2.is_some()
            {
                commands.entity(entity_1).despawn_recursive();
            }

            // Despawn 2
            if
                asteroid_1.is_some() && player_2.is_some() && shield_active_2.is_none() ||
                bullet_1.is_some() && asteroid_2.is_some()
            {
                commands.entity(entity_2).despawn_recursive();
            }

            // Bounce
            if
                asteroid_1.is_some() && asteroid_2.is_some() ||
                asteroid_1.is_some() && player_2.is_some() && shield_active_2.is_some() ||
                player_1.is_some() && shield_active_2.is_some() && asteroid_2.is_some()
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
                dbg!(velocity_1_x_new);
                velocity_1.x = velocity_1_x_new;
                velocity_1.y = velocity_1_y_new;
                velocity_2.x = velocity_2_x_new;
                velocity_2.y = velocity_2_y_new;
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

// Returns the new X and Y velocities of an object after a bounce
fn velocity_after_bounce(
    // Get position, velocity and mass of both entities
    x1: f32,
    y1: f32,
    xv1: f32,
    yv1: f32,
    m1: f32,
    x2: f32,
    y2: f32,
    xv2: f32,
    yv2: f32,
    m2: f32,
    // Returns new x and y velocities
) -> (f32, f32, f32, f32) {
    let mut t1 = (yv1/xv1).atan(); // Theta, ent 1
    if xv1 < 0.0 { t1 += PI; }
    let mut t2 = (yv2/xv2).atan(); // Theta, ent 2
    if xv2 < 0.0 { t2 += PI; }
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