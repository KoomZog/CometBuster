use bevy::prelude::*;
use crate::c_appstate::AppState;
use crate::c_bundles::{BulletBundle, ShieldBundle};
use crate::c_movement_and_collisions::{Angle, CollisionType, Mass, Velocity};
use crate::c_tags::{Player, Shield};
use crate::c_chargelevel::ChargeLevel;
use crate::c_shipstats::{Energy, ShipStats};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(control.in_set(OnUpdate(AppState::InGame)))
        ;
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
        &Transform,
        &Energy,
        &mut ChargeLevel,
        With<Player>,
    )>,
    mut query_shield: Query<(Entity, With<Shield>)>,
) {
    for (entity, mut velocity, ship_stats, mut angle, transform, energy, mut charge_level, _) in query.iter_mut() {

        // Activate Shield
        if keyboard_input.just_pressed(ship_stats.controls.shield) && energy.0 > 20. {
            let shield_entity = commands
            .spawn(ShieldBundle {
                ..Default::default()
            })
            .id();

            commands.entity(entity).push_children(&[shield_entity])
            .insert(CollisionType::Shield);
        }
        // Deactivate Shield
        if keyboard_input.just_released(KeyCode::Z) {
            commands.entity(entity).insert(CollisionType::Ship);
            for (shield_entity, _) in query_shield.iter_mut() {
                commands.entity(shield_entity).despawn_recursive();
            }
        }

        // Rotation
        if keyboard_input.pressed(ship_stats.controls.turn_left) {
            angle.0 += ship_stats.turn_rate * time.delta_seconds();
        }
        if keyboard_input.pressed(ship_stats.controls.turn_right) {
            angle.0 -= ship_stats.turn_rate * time.delta_seconds();
        }

        // Acceleration
        if keyboard_input.pressed(ship_stats.controls.accelerate) {
            velocity.x += ship_stats.acceleration * angle.0.cos() * time.delta_seconds();
            velocity.y += ship_stats.acceleration * angle.0.sin() * time.delta_seconds();
        }

        // Fire
        if keyboard_input.pressed(ship_stats.controls.fire) {
            charge_level.0 += ship_stats.charge_rate * time.delta_seconds();
            if charge_level.0 > 2.0 {charge_level.0 = 2.0;}
        }
        if keyboard_input.just_released(ship_stats.controls.fire) {
            commands.spawn(BulletBundle {
                ..Default::default()
            })
            .insert(Transform {
                translation: Vec3::new(
                    transform.translation.x + angle.0.cos() * 25.0,
                    transform.translation.y + angle.0.sin() * 25.0,
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