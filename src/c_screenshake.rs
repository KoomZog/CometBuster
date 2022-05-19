use bevy::prelude::*;

#[derive(Component)]
pub struct ScreenShake {
    pub start_time: instant::Instant,
    pub duration: instant::Duration,
    pub amplitude: f32,
}
impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            start_time: instant::Instant::now(),
            duration: instant::Duration::from_secs_f32(0.3),
            amplitude: 4.0,
        }
    }
}