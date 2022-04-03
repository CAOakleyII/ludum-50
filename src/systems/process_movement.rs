use bevy::{prelude::*};

use crate::components::{Velocity, Stateful, State, StateKind};

pub fn process_movement(
    delta_time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>
){
    for (mut transform, velocity) in query.iter_mut() {
        let dt = delta_time.delta_seconds();

        transform.translation.x += velocity.vector.x * dt;
        transform.translation.y += velocity.vector.y * dt;
    }
}

pub fn determine_movement_state(
    mut query: Query<(&mut Stateful, &Velocity)>
){
    for (mut state, velocity) in query.iter_mut() {
        let movement_state;

        if velocity.vector == Vec3::ZERO {
            movement_state = State {
                kind: StateKind::Idle,
                interruptable: true,
                should_loop: true,
                running: false
            };
        } else {
            movement_state = State {
                kind: StateKind::Run,
                interruptable: true,
                should_loop: true,
                running: false
            };
        }
        state.next_states.insert(movement_state);
    }

}