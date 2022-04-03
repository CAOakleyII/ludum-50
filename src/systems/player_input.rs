use bevy::prelude::*;

use crate::components::{Velocity, Speed, Player, Aim, Stateful, StateKind, Direction, DirectionName};

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Speed, &Stateful, &mut Direction), With<Player>>
){
    let (mut velocity, speed, stateful, mut direction) = query.single_mut();

    let mut force_x = 0.0f32;
    let mut force_y = 0.0f32;

    let current_direction_name = direction.name.clone();

    if stateful.current_state.kind != StateKind::MeleeAttack {
        if keyboard_input.pressed(KeyCode::A) {
            direction.name = DirectionName::Left;
            force_x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.name = DirectionName::Right;
            force_x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction.name = DirectionName::Up;
            force_y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.name = DirectionName::Down;
            force_y -= 1.0;
        }
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

    if current_direction_name != direction.name {
        direction.new_direction = true
    } else {
        direction.new_direction = false
    }

    velocity.vector.x = force_x * speed.value;
    velocity.vector.y = force_y * speed.value;
}

pub fn player_combat_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Stateful, With<Player>>
) {
    let mut state = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::J) {
        println!("Melee attack added.");
        let melee_attack = crate::components::State {
            kind: StateKind::MeleeAttack,
            interruptable: false,
            should_loop: false,
            running: false
        };
        state.next_states.insert(melee_attack);
    }

    if keyboard_input.just_pressed(KeyCode::K) {
        // charging
        let charge_bow_attack = crate::components::State {
            kind: StateKind::ChargeBow,
            interruptable: false,
            should_loop: true,
            running: false
        };
        state.next_states.insert(charge_bow_attack);
    }

    if keyboard_input.just_released(KeyCode::K) {
        state.current_state.interruptable = true;

        let release_bow_attack = crate::components::State {
            kind: StateKind::ReleaseBow,
            interruptable: false,
            should_loop: false,
            running: false // TODO: Make true with seperate interuptlevels
        };
        state.next_states.insert(release_bow_attack);
        // shooot arrow
    }
}

pub fn player_aim(
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut player_query: Query<&mut Aim, With<Player>>
) {
    // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    // get the window that the camera is displaying to
    let mut aim = player_query.single_mut();
    let (camera, camera_transform) = camera_query.single();
    let window = windows.get(camera.window).unwrap();

    if let Some(position) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (position / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        aim.vector.x = world_pos.x;
        aim.vector.y = world_pos.y;
    }
}