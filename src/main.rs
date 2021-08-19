// TO DO - Rough order
// Web export
// Asteroids splitting when destroyed
// Shield bouncing on asteroids, needs mass component
// Levels

// -- Z LAYERS --
// 20 Shield
// 10 Player ship, asteroids, bullets
// 00 Background

// Crates
use console_error_panic_hook;
use instant;

use bevy::prelude::*;
use rand::Rng;
use std::panic;

const SHIP_SPRITE: &str = "ship.png";
const SHIELD_SPRITE: &str = "shield.png";
const BACKGROUND_SPRITE: &str = "background.png";
const ASTEROID_1_SPRITE: &str = "asteroid_1.png";
const PI: f32 = std::f32::consts::PI;

struct Materials {
    ship: Handle<ColorMaterial>,
    shield: Handle<ColorMaterial>,
    background: Handle<ColorMaterial>,
    asteroid_1: Handle<ColorMaterial>,
}

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
    app.insert_resource(Msaa { samples: 4 });

    #[cfg(target_arch = "wasm32")]
    app.add_plugins_with(DefaultPlugins, |group| {
        group.disable::<bevy::log::LogPlugin>()
    })
    .add_plugin(bevy_webgl2::WebGL2Plugin);
    
    app.insert_resource(WindowDescriptor {
        title: "CometBuster".to_string(),
        width: 1280.0,
        height: 720.0,
        cursor_visible: false,
        ..Default::default()
    });
    
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(DefaultPlugins);

    app.add_startup_system(setup.system())
    .add_startup_stage(
        "setup_resources",
        SystemStage::single(setup_resources.system()),
    )
    .add_system(control.system())
    .add_system(respawn_player.system())
    .add_system(respawn_asteroid.system())
    .add_system(despawn_after_lifetime.system())
    .add_system(collision_detection.system())
    .add_system(gain_energy.system())
    .add_system(drain_energy.system())
    .add_system(activate_shield.system())
    .add_system(deactivate_shield.system())
    //.add_system(pivot_shield.system())
    .add_system(movement_translation.system())
    .add_system(movement_rotation.system())
    .add_system(edge_looping.system())
    .add_system(normalize_angle.system())
    .run();
}

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
        sprite: Sprite::new(Vec2::new(1280., 720.)),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}

// -- SYSTEMS --

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
                .insert(SpawnTime(instant::Instant::now()))
                .insert(Lifetime(instant::Duration::new(1, 0)));
        }
    }
}

fn respawn_player (
    mut commands: Commands,
    materials: Res<Materials>,
    mut query: Query<With<Player>>
){
    if let Ok(_) = query.single_mut() {
    } else {
        commands
        .spawn_bundle(SpriteBundle {
            material: materials.ship.clone(),
            sprite: Sprite::new(Vec2::new(60., 60.)),
            transform: Transform {
                translation: Vec3::new(rf32(0., 800.), rf32(0., 450.), 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity::default())
        .insert(Angle::default())
        .insert(Acceleration::default())
        .insert(Radius(25.))
        .insert(Energy::default());
    }
}

fn respawn_asteroid (
    mut commands: Commands,
    materials: Res<Materials>,
    mut query: Query<With<Asteroid>>
){
    if let Ok(_) = query.single_mut() {
    } else {
        commands
        .spawn_bundle(SpriteBundle {
            material: materials.asteroid_1.clone(),
            sprite: Sprite::new(Vec2::new(120., 120.)),
            transform: Transform {
                translation: Vec3::new(rf32(0., 800.), rf32(0., 450.), 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Velocity {
            x: rf32(-30., 30.),
            y: rf32(-30., 30.),
        })
        .insert(Radius(60.))
        .insert(Asteroid)
        .insert(AsteroidBig);
    }
}

fn despawn_after_lifetime(
    mut commands: Commands,
    mut query: Query<(Entity, &SpawnTime, &Lifetime)>,
) {
    for (entity, spawn_time, lifetime) in query.iter_mut() {
        if spawn_time.0.elapsed() > lifetime.0 {
            commands.entity(entity).despawn();
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
                commands.entity(asteroid_entity).despawn();
                commands.entity(bullet_entity).despawn();
            }
        }
        
        // Asteroid vs Shield, destroy Asteroid
        if let Ok((&shield_transform, shield_radius, _)) = query_shield.single_mut() {
            let distance_x = (asteroid_transform.translation.x - shield_transform.translation.x).abs();
            let distance_y = (asteroid_transform.translation.y - shield_transform.translation.y).abs();
            let distance = distance_x.hypot(distance_y);
            if distance < shield_radius.0 + asteroid_radius.0 {
                commands.entity(asteroid_entity).despawn();
            }
        }

        // Asteroid vs Player, destroy Player
        if let Ok((player_entity, &player_transform, player_radius, _, _)) = query_player.single_mut() {
            let distance_x = (asteroid_transform.translation.x - player_transform.translation.x).abs();
            let distance_y = (asteroid_transform.translation.y - player_transform.translation.y).abs();
            let distance = distance_x.hypot(distance_y);
            if distance < player_radius.0 + asteroid_radius.0 {
                commands.entity(player_entity).despawn();
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

/*
fn pivot_shield(
    mut query_shield: Query<(&mut Angle, With<Shield>)>,
    mut query_player: Query<(&Angle, With<Player>)>,
){
    if let Ok((mut angle_shield, _)) = query_shield.single_mut() {
        if let Ok((angle_player, _)) = query_player.single_mut() {
            angle_shield.0 = -angle_player.0;
        }
    }
}
*/

fn movement_translation(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn movement_rotation(mut query: Query<(&mut Transform, &Angle)>) {
    for (mut transform, angle) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(angle.0);
    }
}

fn edge_looping(mut query: Query<&mut Transform>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    for mut transform in query.iter_mut() {
        if transform.translation.x < -window.width() / 2. {
            transform.translation.x += window.width();
        }
        if transform.translation.x > window.width() / 2. {
            transform.translation.x -= window.width();
        }
        if transform.translation.y < window.height() / 2. {
            transform.translation.y += window.height();
        }
        if transform.translation.y > window.height() / 2. {
            transform.translation.y -= window.height();
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