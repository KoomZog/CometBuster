use bevy::prelude::*;
use crate::consts::*;
use crate::helpers::*;
use crate::c_appstate::AppState;
use crate::c_sprites::{AsteroidSize, SpriteType, Textures};
use crate::c_events::{EvSpawnAsteroidFragments, EvCmpSpawnSprites};
use crate::c_chargelevel::ChargeLevel;
use crate::c_tags::{GridSprite, Player};
use crate::c_movement_and_collisions::{CollisionType, Velocity};
use crate::c_bundles::{AsteroidBigBundle, AsteroidMediumBundle, AsteroidSmallBundle, ShipBundle};
use crate::c_lifetime_spawntime::{Lifetime, SpawnTime};

pub struct SpawnDespawnPlugin;

impl Plugin for SpawnDespawnPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(despawn_after_lifetime))
        .add_system_set(SystemSet::on_enter(AppState::SpawnStart).with_system(spawn_background_player_asteroids))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(respawn_player))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(spawn_sprite_grid))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(spawn_asteroid_fragments))
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

fn spawn_background_player_asteroids (
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    textures: Res<Textures>,
){
    commands.spawn_bundle(SpriteBundle {
        texture: textures.background.clone_weak(),
        sprite: Sprite{
            flip_x: false,
            flip_y: false,
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(WINDOW_WIDTH + 10.0, WINDOW_HEIGHT + 10.0))
        },
        ..Default::default()
    });

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

    state.replace(AppState::InGame).unwrap();
}

fn respawn_player (
    mut commands: Commands,
    query_free_space: Query<(&Transform, &CollisionType)>,
    mut query_player: Query<With<Player>>,
){
    if query_player.iter_mut().len() == 0 {
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

fn spawn_sprite_grid (
    mut commands: Commands,
    mut query: Query<(Entity, &SpriteType, With<EvCmpSpawnSprites>, Option<&AsteroidSize>, Option<&ChargeLevel>)>,
    textures: Res<Textures>,
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
        let texture: Handle<Image>;
        let z_position: f32;
        if sprite_type.is_ship() {
            texture = textures.ship.clone_weak();
            sprite_size = 60.0;
            z_position = 20.0;
        }
        else if let Some(asteroid_size) = asteroid_size {
            texture = textures.asteroid_1.clone_weak();
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
            texture = textures.shield.clone_weak();
            sprite_size = 72.0;
            z_position = 30.0;
        }
        else if let Some(charge_level) = charge_level {
            texture = textures.bullet.clone_weak();
            sprite_size = 30.0 * (1.0 + 1.8 * (charge_level.0 / 1.0).floor());
            z_position = 10.0;
        }
        else { // Always initialize values. TODO: Make this sprite something obvious for debugging
            texture = textures.ship.clone_weak();
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
                texture: texture.clone(),
                sprite: Sprite{
                    custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                    color: Color::rgb(1.0, 1.0, 1.0),
                    flip_x: false,
                    flip_y: false
                },
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