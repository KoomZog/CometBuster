use bevy::prelude::*;
use crate::consts::*;
use crate::c_appstate::AppState;
use crate::c_movement_and_collisions::{Angle, Velocity};
use crate::c_tags::{Bullet, GridSprite, Original};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, movement_translation.run_if(in_state(AppState::InGame)))
        .add_systems(Update, movement_rotation.run_if(in_state(AppState::InGame)))
        .add_systems(Update, edge_looping.run_if(in_state(AppState::InGame)))
        .add_systems(Update, bullet_direction_to_angle.run_if(in_state(AppState::InGame)))
        .add_systems(Update, normalize_angle.run_if(in_state(AppState::InGame)))
        ;
    }
}

fn movement_translation(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, With<Original>)>
) {
    for (mut transform, velocity, _) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn movement_rotation(
    mut query_root: Query<(&Angle, &Children, With<Original>)>,
    mut query_children: Query<(&mut Transform, With<GridSprite>)>
) {
    for (angle, children, _) in query_root.iter_mut() {
        for child in children.into_iter() {
            if let Ok((mut transform_child, _)) = query_children.get_mut(*child) {
                transform_child.rotation = Quat::from_rotation_z(angle.0);
            }
        }
    }
}

fn edge_looping(
    mut query: Query<(&mut Transform, With<Original>)>,
) {
    for (mut transform, _) in query.iter_mut() {
        if transform.translation.x < -WINDOW_WIDTH / 2. {
            transform.translation.x += WINDOW_WIDTH;
        }
        if transform.translation.x > WINDOW_WIDTH / 2. {
            transform.translation.x -= WINDOW_WIDTH;
        }
        if transform.translation.y < WINDOW_HEIGHT / 2. {
            transform.translation.y += WINDOW_HEIGHT;
        }
        if transform.translation.y > WINDOW_HEIGHT / 2. {
            transform.translation.y -= WINDOW_HEIGHT;
        }
    }
}

fn bullet_direction_to_angle (
    mut query: Query<(&mut Angle, &Velocity, With<Bullet>)>
) {
    for (mut angle, velocity, _) in query.iter_mut() {
        angle.0 = (velocity.y / velocity.x).atan();
        if velocity.x < 0.0 { angle.0 += PI}
    }
}

fn normalize_angle(mut query: Query<&mut Angle>) {
    for mut angle in query.iter_mut() {
        if angle.0 > 2. * PI || angle.0 < -2. * PI {
            angle.0 = angle.0 % (2. * PI);
        }
        if angle.0 < 0. {
            angle.0 += 2. * PI;
        }
    }
}