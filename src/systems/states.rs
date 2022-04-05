use bevy::{prelude::{Query, With, Commands, Entity}, math::Vec3};

use crate::components::{Stateful, Velocity, StateKind, State, Rooted, Grounded};

pub fn process_state_queues(
    mut commands: Commands,
    mut query: Query<(&mut Stateful, Entity)>
) {
    for (mut stateful, entity) in query.iter_mut() {
        let mut new_state = &stateful.current_state;
        let mut is_new_state = false;

        for state in stateful.next_states.iter() {
            if (state.kind != stateful.current_state.kind && 
                (stateful.current_state.interruptable || !stateful.current_state.running)) || 
                state.kind == StateKind::Death 
            {
                new_state = state;
                is_new_state = true;
                if state.should_root {
                    commands.entity(entity).insert(Rooted { ..Default::default() });
                } else {
                    commands.entity(entity).remove::<Rooted>();
                }
                break;
            }
        }

        stateful.current_state = new_state.to_owned();
        stateful.new_state = is_new_state;
        if stateful.new_state {
            stateful.current_state.running = true;
        }

        stateful.next_states.clear();
    }
}

pub fn determine_movement_state(
    mut query: Query<(&mut Stateful, &Velocity), With<Grounded>>
){
    for (mut state, velocity) in query.iter_mut() {
        let movement_state;

        if velocity.vector == Vec3::ZERO {
            movement_state = State {
                kind: StateKind::Idle,
                interruptable: true,
                should_loop: true,
                running: false,
                should_root: false
            };
        } else {
            movement_state = State {
                kind: StateKind::Run,
                interruptable: true,
                should_loop: true,
                running: false,
                should_root: false
            };
        }
        state.next_states.insert(movement_state);
    }
}