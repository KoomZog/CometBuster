use bevy::prelude::*;
use crate::c_appstate::AppState;
//use crate::consts::*;
//use crate::c_sprites::Textures;
use crate::c_tags::CameraWorld;

pub struct SetupWorldPlugin;

impl Plugin for SetupWorldPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_world)
        ;
    }
}

fn setup_world (
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
//    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default())
    .insert(CameraWorld);

/*
commands.insert_resource(Textures {
    ship: asset_server.load(SHIP_SPRITE),
    shield: asset_server.load(SHIELD_SPRITE),
    bullet: asset_server.load(BULLET_SPRITE),
    asteroid_1: asset_server.load(ASTEROID_1_SPRITE),
    background: asset_server.load(BACKGROUND_SPRITE),
    color_gradients: asset_server.load(TEXTURE_SPRITE),
});
*/

    next_state.set(AppState::SpawnStart);
}