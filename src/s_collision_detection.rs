use bevy::prelude::*;
use crate::helpers::*;
use crate::c_appstate::AppState;
use crate::c_chargelevel::ChargeLevel;
use crate::c_events::{EvSpawnAsteroidFragments, EvShieldCollision};
use crate::c_lifetime_spawntime::SpawnTime;
use crate::c_movement_and_collisions::{CollisionType, Mass, Radius, Velocity};
use crate::c_sprites::AsteroidSize;

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(collision_detection))
        ;
    }
}

fn collision_detection (
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Radius, &Transform, &mut Velocity, &Mass, &CollisionType, Option<&AsteroidSize>, Option<&ChargeLevel>, Option<&SpawnTime>)>,
    mut spawn_asteroid_fragments_writer: EventWriter<EvSpawnAsteroidFragments>,
    mut shield_collision_writer: EventWriter<EvShieldCollision>,
//    mut bounce_effect_writer: EventWriter<EvSpawnBounceEffect>,
) {
    let mut iter = query.iter_combinations_mut();

    while let Some([
        (entity_1, radius_1, transform_1, mut velocity_1, mass_1, collision_type_1, asteroid_size_1, charge_level_1, spawn_time_1),
        (entity_2, radius_2, transform_2, mut velocity_2, mass_2, collision_type_2, asteroid_size_2, charge_level_2, spawn_time_2)
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
            
            // Bullet vs Asteroid -> despawn both or bounce
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
                        radius_1.0,
                    );
                }
            }

            // Asteroid vs Asteroid -> Bounce
            else if
            collision_type_1.is_asteroid() && collision_type_2.is_asteroid()
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
                    radius_1.0,
                );
            }

            // Shield vs anything -> Bounce
            else if
            collision_type_1.is_shield() && collision_type_2.is_asteroid() ||
            collision_type_1.is_asteroid() && collision_type_2.is_shield() ||
            collision_type_1.is_shield() && collision_type_2.is_bullet() ||
            collision_type_1.is_bullet() && collision_type_2.is_shield()
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
                    radius_1.0,
                );

                let shield_transform: &Transform;
                let other_transform: &Transform;
                if collision_type_1.is_shield() {
                    shield_transform = transform_1;
                    other_transform = transform_2;
                } else {
                    shield_transform = transform_2;
                    other_transform = transform_1;
                }
                shield_collision_writer.send(EvShieldCollision{
                    shield_position: Vec2::new(shield_transform.translation.x, shield_transform.translation.y),
                    other_position: Vec2::new(other_transform.translation.x, other_transform.translation.y)
                });
            }

            // Bullet vs Ship -> Despawn both or bounce
            else if
            collision_type_1.is_ship() && collision_type_2.is_bullet() ||
            collision_type_1.is_bullet() && collision_type_2.is_ship()
            {
                let bullet_charge: &ChargeLevel;
                let bullet_spawn_time: &SpawnTime;
                if collision_type_1.is_bullet() {
                    bullet_charge = charge_level_1.unwrap();
                    bullet_spawn_time = spawn_time_1.unwrap();
                } else {
                    bullet_charge = charge_level_2.unwrap();
                    bullet_spawn_time = spawn_time_2.unwrap();
                }
                if bullet_spawn_time.0.elapsed().as_secs_f32() > 0.2 {
                    if bullet_charge.0 > 1.0 {
                        commands.entity(entity_1).despawn_recursive();
                        commands.entity(entity_2).despawn_recursive();
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
                            radius_1.0,
                        );
                    }
                }
            }
        }
    }
}