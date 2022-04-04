use bevy::{prelude::{Query, Res, Handle, With, Transform, Commands, Entity}, sprite::{TextureAtlas, TextureAtlasSprite}, math::Vec3};

use crate::{components::{Stateful, Direction, Player, DirectionName, Velocity, StateKind, State, Rooted, Grounded}, resources::PlayerAnimations};

pub fn process_state_queues(
    mut commands: Commands,
    mut query: Query<(&mut Stateful, Entity)>
) {
    for (mut stateful, entity) in query.iter_mut() {
        let mut new_state = &stateful.current_state;
        let mut is_new_state = false;

        for state in stateful.next_states.iter() {
            if  state.kind != stateful.current_state.kind && 
                (stateful.current_state.interruptable || !stateful.current_state.running) {
                new_state = state;
                is_new_state = true;
                if state.should_root {
                    commands.entity(entity).insert(Rooted);
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

pub fn animate_player_states(
    player_animations: Res<PlayerAnimations>,
    mut query: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &Stateful, &Direction), With<Player>>
) {
    for (mut handle, mut sprite, stateful, direction) in query.iter_mut() {
        if !stateful.new_state && !direction.new_direction {
            continue
        }

        *handle = player_animations.animation_map.get(&stateful.current_state.kind).unwrap().get(&direction.name).unwrap().clone();

        sprite.index = 0 as usize;
        if &direction.name == &DirectionName::Left {
            sprite.flip_x = true;
            // x = x * -1
        } else {
            sprite.flip_x = false;
        }
    }
}