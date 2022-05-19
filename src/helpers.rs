use bevy::prelude::*;
use rand::Rng;

use crate::consts::*;
use crate::c_movement_and_collisions::Velocity;
use crate::c_screenshake::ScreenShake;
use crate::c_lifetime_spawntime::{Lifetime, SpawnTime};

// Returns a random f32 from FIRST_ARGUMENT to SECOND_ARGUMENT, not including SECOND_ARGUMENT
pub fn rf32(low: f32, high: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen::<f32>() * (high - low) + low;
}

// Returns a random position that is not currently occupied by an entity with a CollsionType component
pub fn random_free_position(
    position_vec: &Vec<Vec2>
) -> Vec2 {
    let mut position_free = false;
    let mut x_pos: f32 = 0.0;
    let mut y_pos: f32 = 0.0;
    while position_free == false {
        position_free = true;
        x_pos = rf32(0.0, WINDOW_WIDTH);
        y_pos = rf32(0.0, WINDOW_HEIGHT);
        for position in position_vec.iter() {
            if shortest_distance(position.x, position.y, x_pos, y_pos) < 200.0 {
                position_free = false;
            }
        }
    }
    return Vec2::new(x_pos, y_pos);
}

// Returns the shortest distance between entities, taking edge looping into account
pub fn shortest_distance (x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x_dist_1 = (x1 - x2).abs();
    let x_dist_2 = (x1 - (x2 + WINDOW_WIDTH)).abs();
    let x_dist_3 = (x1 - (x2 - WINDOW_WIDTH)).abs();
    let x_min = f32::min(x_dist_1, f32::min(x_dist_2, x_dist_3));

    let y_dist_1 = (y1 - y2).abs();
    let y_dist_2 = (y1 - (y2 + WINDOW_HEIGHT)).abs();
    let y_dist_3 = (y1 - (y2 - WINDOW_HEIGHT)).abs();
    let y_min = f32::min(y_dist_1, f32::min(y_dist_2, y_dist_3));

    return x_min.hypot(y_min);
}

// Returns the new X and Y velocities of entities after they bounce
pub fn collision_bounce(
    commands: &mut Commands,
    // Get position, velocity and mass of both entities
    t1: Vec3,
    v1: &mut Velocity,
    m1: f32,
    mut t2: Vec3,
    v2: &mut Velocity,
    m2: f32,
    time: &Res<Time>,
    r1: f32,
) {
    // Check if the entities are moving towards each other
    if shortest_distance(
        t1.x + time.delta_seconds() * v1.x,
        t1.y + time.delta_seconds() * v1.y,
        t2.x + time.delta_seconds() * v2.x,
        t2.y + time.delta_seconds() * v2.y,
    ) <
        shortest_distance(t1.x, t1.y, t2.x, t2.y)
    {
        if (t2.x + WINDOW_WIDTH - t1.x).abs() < (t2.x - t1.x).abs() { t2.x += WINDOW_WIDTH; }
        if (t2.x - WINDOW_WIDTH - t1.x).abs() < (t2.x - t1.x).abs() { t2.x -= WINDOW_WIDTH; }
        if (t2.y + WINDOW_HEIGHT - t1.y).abs() < (t2.y - t1.y).abs() { t2.y += WINDOW_HEIGHT; }
        if (t2.y - WINDOW_HEIGHT - t1.y).abs() < (t2.y - t1.y).abs() { t2.y -= WINDOW_HEIGHT; }

        let mut th1 = (v1.y / v1.x).atan(); // Theta, ent 1
        if v1.x < 0.0 { th1 += PI; } // .atan() can only calculate an angle, not which direction along that angle
        let mut th2 = (v2.y / v2.x).atan(); // Theta, ent 2
        if v2.x < 0.0 { th2 += PI; } // .atan() can only calculate an angle, not which direction along that angle
        let vt1 = v1.x.hypot(v1.y).abs(); // Velocity Total, ent 1
        let vt2 = v2.x.hypot(v2.y).abs(); // Velocity Total, ent 2
        let mut th12 = ((t2.y-t1.y)/(t2.x-t1.x)).atan(); // Theta between the entities
        if t2.x < t1.x { th12 += PI; } // .atan() can only calculate an angle, not which direction along that angle

        let v1_start = v1.clone();

        let loss_factor: f32 = 0.95;
        // https://en.wikipedia.org/wiki/Elastic_collision - Two-dimensional collision with two moving objects
        v1.x = loss_factor * (vt1 * (th1-th12).cos() * ( m1 - m2 ) + 2.0 * m2 * vt2 * ( th2 - th12 ).cos() ) / ( m1 + m2 ) * th12.cos() + vt1 * ( th1 - th12 ).sin() * ( th12 + PI / 2.0 ).cos();
        v1.y = loss_factor * (vt1 * (th1-th12).cos() * ( m1 - m2 ) + 2.0 * m2 * vt2 * ( th2 - th12 ).cos() ) / ( m1 + m2 ) * th12.sin() + vt1 * ( th1 - th12 ).sin() * ( th12 + PI / 2.0 ).sin();
        v2.x = loss_factor * (vt2 * (th2-th12).cos() * ( m2 - m1 ) + 2.0 * m1 * vt1 * ( th1 - th12 ).cos() ) / ( m2 + m1 ) * th12.cos() + vt2 * ( th2 - th12 ).sin() * ( th12 + PI / 2.0 ).cos();
        v2.y = loss_factor * (vt2 * (th2-th12).cos() * ( m2 - m1 ) + 2.0 * m1 * vt1 * ( th1 - th12 ).cos() ) / ( m2 + m1 ) * th12.sin() + vt2 * ( th2 - th12 ).sin() * ( th12 + PI / 2.0 ).sin();

        let change_of_momentum: f32 = m1 * (v1_start.x - v1.x).hypot(v1_start.y - v1.y);
        commands.spawn().insert(ScreenShake{amplitude: (0.0 + change_of_momentum / 5000.0).min(10.0), ..Default::default()});

        let contact_point: Vec2 = Vec2::new(t1.x + th12.cos() * r1, t1.y + th12.sin() * r1);
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite { color: Color::rgb(1.0, 1.0, 1.0), flip_x: false, flip_y: false, custom_size: Some(Vec2::new(5.0,5.0)) },
            transform: Transform {
                translation: Vec3::new(
                    contact_point.x,
                    contact_point.y,
                    100.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpawnTime(instant::Instant::now()))
        .insert(Lifetime(instant::Duration::from_secs_f32(1.0)));
    }
}