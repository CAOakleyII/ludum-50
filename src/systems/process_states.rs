use bevy::{prelude::{Query, Res, Handle, With, Transform}, sprite::TextureAtlas};

use crate::{components::{Stateful, Direction, Player, DirectionName}, resources::PlayerAnimations};

pub fn process_state_queues(
    mut query: Query<&mut Stateful>
) {
    for mut stateful in query.iter_mut() {
        let mut new_state = &stateful.current_state;
        let mut is_new_state = false;

        for state in stateful.next_states.iter() {
            if state.interruptable {
                new_state = state;
                is_new_state = true;
                break;
            }
        }

        stateful.current_state = new_state.to_owned();
        stateful.new_state = is_new_state;
        stateful.next_states.clear();
    }
}

pub fn animate_player_states(
    player_animations: Res<PlayerAnimations>,
    mut query: Query<(&mut Handle<TextureAtlas>, &mut Transform, &Stateful, &Direction), With<Player>>
) {
    for (mut handle, mut transform, stateful, direction) in query.iter_mut() {
        if !stateful.new_state && !direction.new_direction {
            continue
        }

        *handle = player_animations.animation_map.get(&stateful.current_state.kind).unwrap().get(&direction.name).unwrap().clone();
        if &direction.name == &DirectionName::Left {
            transform.scale.x = -1.0;
            // x = x * -1
        } else {
            transform.scale.x = 1.0;
        }
    }
}
    