use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
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
use crate::material_shield::MaterialShield;
use crate::material_basic::MaterialBasic;

pub struct SpawnDespawnPlugin;

impl Plugin for SpawnDespawnPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, despawn_after_lifetime.run_if(in_state(AppState::InGame)))
        .add_systems(Update, spawn_background_player_asteroids.run_if(in_state(AppState::SpawnStart)))
        .add_systems(Update, respawn_player.run_if(in_state(AppState::InGame)))
        .add_systems(Update, spawn_sprite_grid.run_if(in_state(AppState::InGame)))
        .add_systems(Update, spawn_asteroid_fragments.run_if(in_state(AppState::InGame)))
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
    mut next_state: ResMut<NextState<AppState>>,
    textures: Res<Textures>,
){
    commands.spawn(SpriteBundle {
        texture: textures.background.clone_weak(),
        sprite: Sprite{
            flip_x: false,
            flip_y: false,
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(WINDOW_WIDTH + 10.0, WINDOW_HEIGHT + 10.0)),
            ..Default::default()
        },
        ..Default::default()
    });

    let mut positions = Vec::<Vec2>::new();
    
    positions.push(Vec2::default());
    commands.spawn(ShipBundle {
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
        commands.spawn(AsteroidBigBundle {
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

    next_state.set(AppState::InGame);
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
        commands.spawn(ShipBundle {
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
    mut res_meshes: ResMut<Assets<Mesh>>,
    mut res_material_shield: ResMut<Assets<MaterialShield>>,
    mut res_material_basic: ResMut<Assets<MaterialBasic>>,
    mut query: Query<(Entity, &SpriteType, With<EvCmpSpawnSprites>, Option<&AsteroidSize>, Option<&ChargeLevel>)>,
    textures: ResMut<Textures>,
){
    for (entity, sprite_type, _ev_cmp_spawn_sprites, asteroid_size, charge_level) in query.iter_mut() {
        commands.entity(entity)
        .remove::<EvCmpSpawnSprites>();

        // -- Z LAYERS --
        // 30 Shield
        // 20 Ship
        // 10 Asteroids, bullets
        // 00 Background

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

                if sprite_type.is_ship() {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: res_meshes.add(Mesh::from(shape::Quad { size: Vec2::new(60.0, 60.0), flip: false })).into(),
                        material: res_material_basic.add(MaterialBasic {
                            texture: Some(textures.ship.clone_weak()),
                        }),
                        transform: Transform {
                            translation: Vec3::new(x_factor * WINDOW_WIDTH, y_factor * WINDOW_HEIGHT, 20.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(GridSprite)
                    .id()
                }
                else if let Some(asteroid_size) = asteroid_size {
                    if asteroid_size.is_big() {
                        commands.spawn(MaterialMesh2dBundle {
                            mesh: res_meshes.add(Mesh::from(shape::Quad { size: Vec2::new(180.0, 180.0), flip: false })).into(),
                            material: res_material_basic.add(MaterialBasic {
                                texture: Some(textures.asteroid_1.clone_weak()),
                            }),
                            transform: Transform {
                                translation: Vec3::new(x_factor * WINDOW_WIDTH, y_factor * WINDOW_HEIGHT, 20.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(GridSprite)
                        .id()
                    }
                    else if asteroid_size.is_medium() {
                        commands.spawn(MaterialMesh2dBundle {
                            mesh: res_meshes.add(Mesh::from(shape::Quad { size: Vec2::new(80.0, 80.0), flip: false })).into(),
                            material: res_material_basic.add(MaterialBasic {
                                texture: Some(textures.asteroid_1.clone_weak()),
                            }),
                            transform: Transform {
                                translation: Vec3::new(x_factor * WINDOW_WIDTH, y_factor * WINDOW_HEIGHT, 20.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(GridSprite)
                        .id()
                    }
                    else {
                        commands.spawn(MaterialMesh2dBundle {
                            mesh: res_meshes.add(Mesh::from(shape::Quad { size: Vec2::new(36.0, 36.0), flip: false })).into(),
                            material: res_material_basic.add(MaterialBasic {
                                texture: Some(textures.asteroid_1.clone_weak()),
                            }),
                            transform: Transform {
                                translation: Vec3::new(x_factor * WINDOW_WIDTH, y_factor * WINDOW_HEIGHT, 20.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(GridSprite)
                        .id()
                    }
                }
                else if sprite_type.is_shield() {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: res_meshes.add(Mesh::from(shape::Quad { size: Vec2::new(72.0, 72.0), flip: false })).into(),
                        material: res_material_shield.add(MaterialShield {
                            texture_gradient: Some(textures.color_gradients.clone_weak()),
                            ..Default::default()
                        }),
                        transform: Transform {
                            translation: Vec3::new(x_factor * WINDOW_WIDTH, y_factor * WINDOW_HEIGHT, 30.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(GridSprite)
                    .id()
                }
                else if let Some(charge_level) = charge_level {
                    let quad_size = 30.0 * (1.0 + 1.8 * (charge_level.0 / 1.0).floor());
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: res_meshes.add(Mesh::from(shape::Quad { size: Vec2::new(quad_size, quad_size), flip: false })).into(),
                        material: res_material_basic.add(MaterialBasic {
                            texture: Some(textures.bullet.clone_weak()),
                        }),
                        transform: Transform {
                            translation: Vec3::new(x_factor * WINDOW_WIDTH, y_factor * WINDOW_HEIGHT, 10.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(GridSprite)
                    .id()
                }
                else { // Always initialize values. TODO: Make this sprite something obvious for debugging
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: res_meshes.add(Mesh::from(shape::Quad { size: Vec2::new(60.0, 60.0), flip: false })).into(),
                        material: res_material_basic.add(MaterialBasic {
                            texture: Some(textures.ship.clone_weak()),
                        }),
                        transform: Transform {
                            translation: Vec3::new(x_factor * WINDOW_WIDTH, y_factor * WINDOW_HEIGHT, 20.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(GridSprite)
                    .id()
                }
            ).collect();

        commands.entity(entity).push_children(&sprite_grid);
    }
}

/*
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

        let quad_size: f32;
        let texture: Handle<Image>;
        let z_position: f32;
        if sprite_type.is_ship() {
            texture = textures.ship.clone_weak();
            quad_size = 60.0;
            z_position = 20.0;
        }
        else if let Some(asteroid_size) = asteroid_size {
            texture = textures.asteroid_1.clone_weak();
            if asteroid_size.is_big() {
                quad_size = 180.0;
                z_position = 10.0;
            }
            else if asteroid_size.is_medium() {
                quad_size = 80.0;
                z_position = 10.0;
            }
            else {
                quad_size = 36.0;
                z_position = 10.0;
            }
        }
        else if sprite_type.is_shield() {
            texture = textures.shield.clone_weak();
            quad_size = 72.0;
            z_position = 30.0;
        }
        else if let Some(charge_level) = charge_level {
            texture = textures.bullet.clone_weak();
            quad_size = 30.0 * (1.0 + 1.8 * (charge_level.0 / 1.0).floor());
            z_position = 10.0;
        }
        else { // Always initialize values. TODO: Make this sprite something obvious for debugging
            texture = textures.ship.clone_weak();
            quad_size = 200.0;
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
            commands.spawn(SpriteBundle {
                texture: texture.clone(),
                sprite: Sprite{
                    custom_size: Some(Vec2::new(quad_size, quad_size)),
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
*/

fn spawn_asteroid_fragments (
    mut commands: Commands,
    mut spawn_asteroid_fragment_reader: EventReader<EvSpawnAsteroidFragments>,
) {
    let added_velocity = 80.0;
    let retained_velocity_factor = 0.9;
    for event in spawn_asteroid_fragment_reader.read() {
        let start_angle = rf32(0.0, 2.0 * PI / 3.0);
        if event.asteroid_size_destroyed.is_big() {
            for i in 0..3 {
                let j = i as f32;
                let spawn_circle_radius: f32 = AsteroidBigBundle::default().physics_object.radius.0 * 0.54;
                let x_pos = event.transform.translation.x + (j * 2.0 * PI / 3.0 + start_angle).cos() * spawn_circle_radius;
                let y_pos = event.transform.translation.y + (j * 2.0 * PI / 3.0 + start_angle).sin() * spawn_circle_radius;
                let x_vel = rf32(-added_velocity, added_velocity);
                let y_vel = rf32(-added_velocity, added_velocity);
                commands.spawn(AsteroidMediumBundle::default())
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
                commands.spawn(AsteroidSmallBundle::default())
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