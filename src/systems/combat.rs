use bevy::{prelude::{Query, Transform, With}, core::Timer};

use crate::components::{Stateful, Player, StateKind, Collidables, CollisionShape, CollisionMasks};

pub fn process_player_combat_states (
    mut query: Query<(&Stateful, &mut Collidables, &Transform), With<Player>>
) {
    for (state, mut collidables, transform) in query.iter_mut() {
        if !state.new_state {
            continue;
        }

        match state.current_state.kind {
            StateKind::MeleeAttack => {
                let frame_length = StateKind::MeleeAttack.player_frame_data().z * 0.1; 
                let melee_hitbox = CollisionShape {
                    mask: CollisionMasks::PlayerAttack,
                    collides_with: CollisionMasks::AI as i32,
                    timer: Timer::from_seconds(frame_length, false),
                    width: 65.0,
                    height: 65.0,
                    ..Default::default()
                };
                collidables.collision_shapes.insert(melee_hitbox.uuid, melee_hitbox);
            },
            _ => {}
        }
    }
}

// pub fn player_melee_collision {
//     mut query: 
// }