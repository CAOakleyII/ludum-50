use bevy::{prelude::*};

use crate::components::{Velocity, Gravity, Grounded, CollisionShape, Ground, Jumping, State, Stateful, StateKind, Direction, Dashing};

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

pub fn process_jumping(
    mut commands: Commands,
    delta_time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Jumping, &mut Stateful, &Gravity, Entity)> 
) {
    for (mut transform, mut jump, mut state, gravity, entity) in query.iter_mut() {
        let dt = delta_time.delta_seconds();
        
        jump.timer.tick(delta_time.delta());
        
        if jump.timer.finished() {
            jump.float_timer.tick(delta_time.delta());
            transform.translation.y += gravity.value * dt;
        } else {
            transform.translation.y +=  jump.force * dt;
            let movement_state = State {
                kind: StateKind::Jump,
                interruptable: true,
                should_loop: true,
                running: false,
                should_root: false
            };
            state.next_states.insert(movement_state);
        }

        if jump.float_timer.finished() {
            let movement_state = State {
                kind: StateKind::Fall,
                interruptable: true,
                should_loop: true,
                running: false,
                should_root: false
            };
            state.next_states.insert(movement_state);
            commands.entity(entity).remove::<Jumping>();
        }
    }
}

pub fn process_dashing(
    mut commands: Commands,
    delta_time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Dashing, &Direction, Entity)> 
) {
    for (mut transform, mut dashing, direction, entity) in query.iter_mut() {
        dashing.timer.tick(delta_time.delta());

        if dashing.timer.finished() {
            let dashing_force = dashing.force * direction.flip_x;
            transform.translation.x += dashing_force;
            commands.entity(entity).remove::<Dashing>();
        }
        
    }
}

pub fn apply_gravity(
    delta_time: Res<Time>,
    mut query: Query<(&mut Transform, &Gravity), Without<Grounded>>
){
    for (mut transform, gravity) in query.iter_mut() {
        let dt = delta_time.delta_seconds();

        transform.translation.y -=  gravity.value * dt;
    }
}

pub fn ground_collision(
    mut commands: Commands,
    query: Query<&CollisionShape, With<Ground>>,
    get_entity: Query<Entity, Without<Grounded>>,
    get_entity_with_grounded: Query<Entity, With<Grounded>>,
) {
    for collision_shape in query.iter() {
        for entity in collision_shape.collisions.iter() {
            if let Ok(e) = get_entity.get(*entity) {
                commands.entity(e)
                    .insert(Grounded);
            }
        }

        for entity in collision_shape.collisions_just_ended.iter() {
            if let Ok(e) = get_entity_with_grounded.get(*entity) {
                commands.entity(e).remove::<Grounded>();
            }
        }
    }
}