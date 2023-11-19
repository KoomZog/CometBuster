// TO DO - Rough order
// *Shield shader
// *Code organization - plugins
// *Shield shader to game
// Give shield shader more glow
// Find/make a brighter ship sprite
// 0.9 - Stageless
// 0.9 - Use global time in shader (group 0 binding 9) 
// Particle effects
// Audio
// GUI
// Levels

use bevy::{
    prelude::*,
    window::*,
//    asset::AssetServerSettings,
};

//use bevy_inspector_egui::quick::WorldInspectorPlugin;

// Add instant crate in modules that need it?
//extern crate instant; // Works exactly like the std::time counterpart on native, but uses JS performance.now() for WASM

use consts::*;
use c_events::*;
use c_sprites::Textures;
use c_appstate::AppState;

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

#[derive(Event, TypePath)]
struct EvSpawnBounceEffect{
//    transform: Transform,
}

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>()
    ;

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "CometBuster".into(),
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }))
/*
cursor: Cursor::new{
    visible: false,
    ..Default::default()},
    ..Default::default()
}
*/

// Watch for assets changes. Must be before DefaultPlugins
//    .insert_resource(AssetServerSettings {
//        watch_for_changes: true,
//        ..default()
//    })
//    .add_plugins(DefaultPlugins)


//    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(EnergyPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(SpawnDespawnPlugin)
    .add_plugins(CollisionDetectionPlugin)
    .add_plugins(ControlPlugin)
    .add_plugins(ScreenShakePlugin)
    .add_plugins(SetupWorldPlugin)
    .add_plugins(MaterialShieldPlugin)
    .add_plugins(MaterialBasicPlugin)
    .add_plugins(PausePlugin)
    ;

    app
    .add_event::<EvSpawnAsteroidFragments>()
    .add_event::<EvSpawnBounceEffect>()
    .add_event::<EvShieldCollision>()
    ;

    app.init_resource::<Textures>()
    ;

    app.run();
}

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        Textures {
            ship: asset_server.load(SHIP_SPRITE),
            shield: asset_server.load(SHIELD_SPRITE),
            bullet: asset_server.load(BULLET_SPRITE),
            asteroid_1: asset_server.load(ASTEROID_1_SPRITE),
            background: asset_server.load(BACKGROUND_SPRITE),
            color_gradients: asset_server.load(TEXTURE_SPRITE),
        }
    }
}