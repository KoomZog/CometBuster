use bevy::prelude::*;
use crate::c_appstate::AppState;
use crate::c_shipstats::{ShipStats, Energy};
use crate::c_tags::Shield;
use crate::c_movement_and_collisions::CollisionType;

pub struct EnergyPlugin;

impl Plugin for EnergyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(drain_energy))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(gain_energy))
        ;
    }
}

fn gain_energy(time: Res<Time>, mut query: Query<(&ShipStats, &mut Energy)>) {
    for (ship_stats, mut energy) in query.iter_mut() {
        energy.0 += ship_stats.shield_regeneration * time.delta_seconds();
        if energy.0 >= 100. {
            energy.0 = 100.;
        }
    }
}

fn drain_energy(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Energy, &CollisionType, &Children)>,
    mut query_shield: Query<(Entity, With<Shield>)>,
) {
    for (entity, mut energy, collision_type, children) in query.iter_mut(){
        if collision_type.is_shield() {
            energy.0 -= 100. * time.delta_seconds();
            if energy.0 <= 0. {
                commands.entity(entity).insert(CollisionType::Ship);
                for child in children.into_iter() {
                    if let Ok((shield_entity, _)) = query_shield.get_mut(*child) {
                        commands.entity(shield_entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

