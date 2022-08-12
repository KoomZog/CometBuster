// TO DO - Rough order
// *Shield shader
// *Code organization - plugins
// *Shield shader to game
// Particle effects
// Audio
// GUI
// Levels

use bevy::{
    prelude::*,
//    asset::AssetServerSettings,
};

// Add instant crate in modules that need it?
//extern crate instant; // Works exactly like the std::time counterpart on native, but uses JS performance.now() for WASM

use consts::*;
use c_appstate::AppState;
use c_events::*;

mod helpers;
mod consts;
mod c_controls;
mod c_bundles;
mod c_tags;
mod c_events;
mod c_sprites;
mod c_chargelevel;
mod c_lifetime_spawntime;
mod c_movement_and_collisions;
mod c_screenshake;
mod c_shipstats;
mod c_appstate;

mod material_shield;
use material_shield::*;
mod material_basic;
use material_basic::*;

mod s_energy;
use s_energy::EnergyPlugin;
mod s_movement;
use s_movement::MovementPlugin;
mod s_spawn_despawn;
use s_spawn_despawn::SpawnDespawnPlugin;
mod s_collision_detection;
use s_collision_detection::CollisionDetectionPlugin;
mod s_control;
use s_control::ControlPlugin;
mod s_screen_shake;
use s_screen_shake::ScreenShakePlugin;
mod s_pause;
use s_pause::PausePlugin;
mod s_setup_world;
use s_setup_world::SetupWorldPlugin;

struct EvSpawnBounceEffect{
//    transform: Transform,
}

fn main() {
    let mut app = App::new();
//    app.insert_resource(Msaa { samples: 4 }); // TODO: Find out what this does. Was in the web template. Does not seem to be needed.

    app.insert_resource(WindowDescriptor {
        title: "CometBuster".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        cursor_visible: false,
        ..Default::default()
    })
    // Watch for assets changes. Must be before DefaultPlugins
//    .insert_resource(AssetServerSettings {
//        watch_for_changes: true,
//        ..default()
//    })
    .add_plugins(DefaultPlugins)
    .add_plugin(EnergyPlugin)
    .add_plugin(MovementPlugin)
    .add_plugin(SpawnDespawnPlugin)
    .add_plugin(CollisionDetectionPlugin)
    .add_plugin(ControlPlugin)
    .add_plugin(ScreenShakePlugin)
    .add_plugin(PausePlugin)
    .add_plugin(SetupWorldPlugin)
    .add_plugin(MaterialShieldPlugin)
    .add_plugin(MaterialBasicPlugin)
    ;

    app
    .add_state(AppState::SetupMaterials)
    ;

    app
    .add_event::<EvSpawnAsteroidFragments>()
    .add_event::<EvSpawnBounceEffect>()
    .add_event::<EvShieldCollision>()
    ;

    app.run();
}