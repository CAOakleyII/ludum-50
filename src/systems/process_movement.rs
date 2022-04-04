use bevy::{prelude::*};

use crate::components::{Velocity, Gravity, Grounded, CollisionShape, Ground, Jumping};

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
    mut query: Query<(&mut Transform, &mut Jumping, Entity)> 
) {
    for (mut transform, mut jump, entity) in query.iter_mut() {
        let dt = delta_time.delta_seconds();

        transform.translation.y +=  jump.force * dt;

        jump.timer.tick(delta_time.delta());

        if jump.timer.finished() {
            commands.entity(entity).remove::<Jumping>();
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
    query: Query<&CollisionShape, With<Ground>>
) {
    for collision_shape in query.iter() {
        for entity in collision_shape.collisions.iter() {
            commands.entity(*entity)
                .insert(Grounded);
        }

        for entity in collision_shape.collisions_just_ended.iter() {
            commands.entity(*entity).remove::<Grounded>();
        }
    }
}