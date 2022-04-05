use bevy::{prelude::{Query, Transform, With, Commands, Without, Entity, Res, DespawnRecursiveExt, Or}, core::{Timer, Time}, math::{Vec3}};

use crate::components::{Stateful, Player, StateKind, CollisionShape, MeleeAttack, Damaged, CurrentHealth, Direction, AI, Ground, State, Hit, Dead};

pub fn process_combat_states (
    mut commands: Commands,
    mut query: Query<(&Stateful, &mut MeleeAttack, Entity), Or<(With<Player>, With<AI>)>>
) {
    for (state, mut melee_attack, entity) in query.iter_mut() {
        if !state.new_state {
            continue;
        }

        match state.current_state.kind {
            StateKind::MeleeAttack => {
                let frame_length = StateKind::MeleeAttack.player_frame_data().z * 0.1;
                let mut double_strike = melee_attack.clone();
                double_strike.width = 32.0;
                double_strike.height = 23.0;
                double_strike.damage = 17.5;
                double_strike.active_frame_length = 0.4;
                double_strike.timer = Timer::from_seconds(0.9, true); // repeating enables multiple strikes, at 0.9 intervals
                double_strike.offset = Vec3::new(10.0, 0.0, 0.0);
                double_strike.knockback_force = 145.0;
                double_strike.full_attack_timer = Timer::from_seconds(frame_length, false);
                double_strike.forward_step = 10.0;
                double_strike.character_id.push(entity);

                commands.spawn()
                    .insert(double_strike);
            },
            StateKind::TripleAttack => {
                let frame_length = StateKind::TripleAttack.player_frame_data().z * 0.1;
                let mut triple_attack = melee_attack.clone();
                triple_attack.width = 20.0;
                triple_attack.height = 20.0;
                triple_attack.damage = 23.0;
                triple_attack.active_frame_length = 0.2;
                triple_attack.timer = Timer::from_seconds(0.9, true); // repeating enables multiple strikes, at 0.9 intervals
                triple_attack.offset = Vec3::new(10.0, 0.0, 0.0);
                triple_attack.knockback_force = 75.0;
                triple_attack.full_attack_timer = Timer::from_seconds(frame_length, false);
                triple_attack.forward_step = 25.0;
                triple_attack.character_id.push(entity);

                commands.spawn()
                    .insert(triple_attack);
            },
            StateKind::StabDash => {
                let frame_length = StateKind::StabDash.player_frame_data().z * 0.1; 

                let mut stab = melee_attack.clone();
                stab.width = 20.0;
                stab.height = 10.0;
                stab.damage = 10.0;
                stab.active_frame_length = 0.3;
                stab.timer = Timer::from_seconds(0.3, false);
                stab.offset = Vec3::new(15.0, 0.0, 0.0);
                stab.knockback_force = 75.0;
                stab.forward_step = 0.0;
                stab.character_id.push(entity);
                stab.full_attack_timer = Timer::from_seconds(frame_length, false);

                commands.spawn()
                    .insert(stab);
            },
            StateKind::AIAttack => {
                melee_attack.cool_down_timer.reset();
                melee_attack.character_id.push(entity);

                let ai_attack = melee_attack.clone();
                commands.spawn()
                    .insert(ai_attack);
            }
            _ => {}
        }
    }
}

pub fn process_melee_attacks (
    mut commands: Commands,
    delta_time: Res<Time>,
    mut query: Query<(&mut MeleeAttack, Entity), (Without<Player>, Without<CollisionShape>, Without<AI>)>,
    mut player_transform_query: Query<(&mut Transform, &Direction), Or<(With<Player>, With<AI>)>>
) {
    for (mut melee_attack, entity) in query.iter_mut() {
        melee_attack.timer.tick(delta_time.delta());
        melee_attack.full_attack_timer.tick(delta_time.delta());

        if melee_attack.full_attack_timer.just_finished() {
            commands.entity(entity).despawn();
        }

        if !melee_attack.timer.just_finished() || melee_attack.full_attack_timer.finished() {
            continue;
        }

        if let Ok((mut character_transform, direction)) = player_transform_query.get_mut(*melee_attack.character_id.first().unwrap()) {
            character_transform.translation.x += melee_attack.forward_step * direction.flip_x;
            let mut transform = character_transform.clone();
            let mut ma = melee_attack.clone();
            transform.translation += melee_attack.offset * direction.flip_x;
            ma.knockback_force *= direction.flip_x;

            let melee_hitbox = CollisionShape {
                mask: melee_attack.mask,
                collides_with: melee_attack.collides_with,
                timer: Timer::from_seconds(melee_attack.active_frame_length, false),
                width: melee_attack.width,
                height: melee_attack.height,
                ..Default::default()
            };
            commands.spawn()
                        .insert(transform)
                        .insert(melee_hitbox)
                        .insert(ma);
        }
        
    }
}

pub fn player_melee_attack_collision (
    mut commands: Commands,
    delta_time: Res<Time>,
    mut query: Query<(&mut MeleeAttack, &CollisionShape), (Without<Player>, Without<AI>)>,
    get_entity: Query<Entity>,
    mut transform_query: Query<&mut Transform>
) {
    for (mut melee_attack, collision_shape) in query.iter_mut() {
        for entity in collision_shape.collisions.iter() {
            if let Ok(e) = get_entity.get(*entity) {
                if let Ok(mut transform) = transform_query.get_component_mut::<Transform>(e) {
                    transform.translation.x += melee_attack.knockback_force * delta_time.delta_seconds();
                }

                if melee_attack.damage_table.contains(&e) {
                    continue;
                }
                commands.entity(e)
                .insert(Damaged(melee_attack.damage));

                melee_attack.damage_table.insert(e);
            }
        }
    }
}

pub fn damage_entity(
    mut commands: Commands,
    mut query: Query<(&mut CurrentHealth, &Damaged, Entity)>
) {
    for (mut current_health, damage, entity) in query.iter_mut() {
        current_health.value -= damage.0;
        commands.entity(entity)
            .insert(Hit(
                Timer::from_seconds(0.1, false),
            ))
            .remove::<Damaged>();
    }
}

pub fn destroy_ai(
    mut commands: Commands,
    mut query: Query<(&CurrentHealth, &mut Stateful, Entity), (With<AI>, Without<Dead>)>,
    mut get_ground: Query<&mut Ground>,
){
    for (current_health, mut state, entity) in query.iter_mut() {
        if current_health.value <= 0.0 {
            for mut ground in get_ground.iter_mut() {
                ground.0 += 50.0;
            }

            let death_state = State {
                kind: StateKind::Death,
                interruptable: false,
                should_loop: false,
                running: false,
                should_root: true
            };
            state.next_states.insert(death_state);
            commands.entity(entity).insert(Dead(Timer::from_seconds(1.0, false)));
        }
    }
}

pub fn process_death(
    mut commands: Commands,
    delta_time: Res<Time>,
    mut query: Query<(&mut Dead, Entity)>
) {
    for (mut dead, entity) in query.iter_mut() {
        dead.0.tick(delta_time.delta());

        if dead.0.just_finished(){
            commands.entity(entity).despawn_recursive();
        }
    }
}