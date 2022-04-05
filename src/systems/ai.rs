use bevy::{prelude::{Query, Transform, With, Without, Commands, Entity, Res}, core::Time};

use crate::components::{Velocity, AI, Player, InAttackRange, Speed, Rooted, AttackRange, Stateful, DirectionName, Direction, StateKind, State, MeleeAttack, Grounded, Dead};

pub fn chase_player(
    mut ai_query: Query<(&mut Velocity, &Transform, &Speed), (With<AI>, Without<Rooted>, Without<InAttackRange>)>,
    player_query: Query<&Transform, With<Player>>
){
    let player_transform = player_query.single();

    for (mut velocity, transform, speed) in ai_query.iter_mut() {
        if player_transform.translation.x < transform.translation.x {
            velocity.vector.x = -1.0 * speed.value;

        } else {
            velocity.vector.x = 1.0 * speed.value;
        }
    }
}

pub fn face_player(
    mut ai_query: Query<( &Transform, &mut Direction), (With<AI>, Without<Rooted>)>,
    player_query: Query<&Transform, With<Player>>
) {
    let player_transform = player_query.single();

    for (transform, mut direction) in ai_query.iter_mut() {
        let current_direction_name = direction.name.clone();
        if player_transform.translation.x < transform.translation.x {
            direction.name = DirectionName::Left;

        } else {
            direction.name = DirectionName::Right;
        }

        if current_direction_name != direction.name {
            direction.new_direction = true
        } else {
            direction.new_direction = false
        }
    }
}

pub fn determine_ai_in_attack_range(
    mut commands: Commands,
    mut ai_query: Query<(&mut Velocity, &Transform, &AttackRange, Entity), With<AI>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();

    for (mut velocity, transform, attack_range, entity) in ai_query.iter_mut() {
        let distance = transform.translation.distance(player_transform.translation).abs();
        if distance <= attack_range.value {
            commands.entity(entity).insert(InAttackRange);
            velocity.vector.x = 0.0;
        } else {
            commands.entity(entity).remove::<InAttackRange>();
        }
    }
}

pub fn tick_ai_cool_downs(
    delta_time: Res<Time>,
    mut ai_query: Query<&mut MeleeAttack, With<AI>>,
) {
    for mut melee_attack in ai_query.iter_mut() {
        melee_attack.cool_down_timer.tick(delta_time.delta());
    }
}

pub fn attack_player(
    mut ai_query: Query<(&mut Stateful, &MeleeAttack), (With<AI>, With<InAttackRange>, With<Grounded>, Without<Dead>)>,
) {
    for (mut state, melee_attack) in ai_query.iter_mut() {
        if melee_attack.cool_down_timer.just_finished() {
            let ai_attack_state = State {
                kind: StateKind::AIAttack,
                interruptable: false,
                should_loop: false,
                running: false,
                should_root: true
            };
            state.next_states.insert(ai_attack_state);
        }
    }
}
