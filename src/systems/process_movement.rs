use bevy::{prelude::*};

use crate::components::{Velocity};

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