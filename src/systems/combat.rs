use std::time::Duration;

use bevy::{prelude::{Query, Transform, With, Commands, Without, Entity, Res, DespawnRecursiveExt}, core::{Timer, Time}};

use crate::components::{Stateful, Player, StateKind, CollisionShape, CollisionMasks, MeleeAttack, Damaged, CurrentHealth, Direction, DirectionName, MaxHealth, AI, Ground};

pub fn process_player_combat_states (
    mut commands: Commands,
    mut query: Query<(&Stateful, &mut MeleeAttack, &Transform, &Direction), With<Player>>
) {
    for (state, melee_attack, transform, direction) in query.iter_mut() {
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
                    width: melee_attack.width,
                    height: melee_attack.height,
                    ..Default::default()
                };

                let mut t = transform.clone();
                let mut offset = 15.0;
                if direction.name == DirectionName::Left {
                    offset = -offset;
                }

                t.translation.x += offset;
                commands.spawn()
                    .insert(melee_hitbox)
                    .insert(t)
                    .insert(melee_attack.clone());
            },
            _ => {}
        }
    }
}

pub fn player_melee_attack_collision (
    mut commands: Commands,
    delta_time: Res<Time>,
    mut query: Query<(&mut MeleeAttack, &CollisionShape), Without<Player>>,
    get_entity: Query<Entity>
) {
    for (mut melee_attack, collision_shape) in query.iter_mut() {
        melee_attack.timer.tick(delta_time.delta());
        if !melee_attack.timer.just_finished() {
            continue;
        }

        for entity in collision_shape.collisions.iter() {
            if let Ok(_) = get_entity.get(*entity) {
                commands.entity(*entity)
                .insert(Damaged(melee_attack.damage));
            }
        }

        melee_attack.timer.set_duration(Duration::from_secs_f32(0.9));
        melee_attack.timer.reset();
    }
}

pub fn damage_entity(
    mut commands: Commands,
    mut query: Query<(&mut CurrentHealth, &Damaged, Entity)>
) {
    for (mut current_health, damage, entity) in query.iter_mut() {
        current_health.value -= damage.0;
        commands.entity(entity).remove::<Damaged>();
    }
}

pub fn destroy_ai(
    mut commands: Commands,
    query: Query<(&CurrentHealth, Entity), With<AI>>,
    mut get_ground: Query<&mut Ground>,
){
    for (current_health, entity) in query.iter() {
        if current_health.value <= 0.0 {
            for mut ground in get_ground.iter_mut() {
                ground.0 += 50.0;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}