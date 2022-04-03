use std::f32::consts;

use bevy::prelude::*;
use crate::{components::{Aim, Direction, DirectionName}};

pub fn determine_direction(
    mut query: Query<(&mut Direction, &Aim, &Transform)>
) {
    for (mut direction, aim, transform) in query.iter_mut() {
        
        // direction.angle = transform.translation.angle_between(aim.vector);
        let (x2, y2) = (aim.vector.x, aim.vector.y);
        let (x1, y1) = (transform.translation.x, transform.translation.y);
        let y = y2 - y1;
        let x = x2 - x1;

        direction.angle = y.atan2(x);
        let current_direction_name = direction.name.clone();

        if direction.angle >= consts::PI * (6.0/8.0) ||
            direction.angle <= consts::PI * -(6.0/8.0) {
                direction.name = DirectionName::Left;
        }

        if direction.angle <= consts::PI * (2.0/8.0) && direction.angle >= 0.0 ||
            direction.angle >= consts::PI * -(2.0/8.0) && direction.angle < 0.0 {
                direction.name = DirectionName::Right;
        }

        if direction.angle > consts::PI * (2.0/8.0) && direction.angle < consts::PI * (6.0/8.0) {
            direction.name = DirectionName::Up;
        }

        if direction.angle < consts::PI * -(2.0/8.0) && direction.angle > consts::PI * -(6.0/8.0) {
            direction.name = DirectionName::Down;
        }

        if current_direction_name != direction.name {
            direction.new_direction = true
        } else {
            direction.new_direction = false
        }
    }
}