use bevy::prelude::*;
use crate::helpers::*;
use crate::c_appstate::AppState;
use crate::c_screenshake::ScreenShake;
use crate::c_tags::CameraWorld;

pub struct ScreenShakePlugin;

impl Plugin for ScreenShakePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, screen_shake.run_if(in_state(AppState::InGame)))
        ;
    }
}

fn screen_shake (
    mut commands: Commands,
    query: Query<(Entity, &ScreenShake)>,
    mut query_camera: Query<(&mut Transform, With<CameraWorld>)>,
) {
    for (mut transform, _) in query_camera.iter_mut() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        for (entity, screen_shake) in query.iter() {
            if screen_shake.start_time.elapsed() > screen_shake.duration {
                commands.entity(entity).despawn();
            } else {
                let current_amplitude: f32 = screen_shake.amplitude * (screen_shake.duration.as_secs_f32() - screen_shake.start_time.elapsed().as_secs_f32()) / screen_shake.duration.as_secs_f32();
                transform.translation.x += rf32(-current_amplitude, current_amplitude);
                transform.translation.y += rf32(-current_amplitude, current_amplitude);
            }
        }
    }
}