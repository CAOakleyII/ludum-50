use bevy::prelude::*;

use crate::components::{Velocity, Speed, Player};

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Speed), With<Player>>
){

    let (mut velocity, speed) = query.single_mut();

    let mut force_x = 0.0f32;
    let mut force_y = 0.0f32;

    if keyboard_input.pressed(KeyCode::A) {
        force_x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        force_x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        force_y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        force_y -= 1.0;
    }

    // Force Normalization
    let length = (force_x * force_x + force_y * force_y).sqrt();
    if length == 0.0 {
        force_x = 0.0;
        force_y = 0.0;
    } else {
        force_x = force_x / length;
        force_y = force_y / length;
    }

    velocity.x = force_x * speed.value;
    velocity.y = force_y * speed.value;
}