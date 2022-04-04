use bevy::core::Time;
use bevy::prelude::{Query, Transform, Res, Entity};
use bevy::sprite::collide_aabb::*;

use crate::components::{Collidables, CollisionShape};

pub fn process_collisions(
    mut collidables_query: Query<(&mut Collidables, &Transform, Entity)>,
){ 
    // For each collidables, compare against other collidables
    let mut combinations = collidables_query.iter_combinations_mut();
    
    while let Some([
        (mut collidables, transform, entity),
        (mut other_collidables, other_transform, other_entity)]) = combinations.fetch_next() 
    {
        for ( _, collision_shape) in collidables.collision_shapes.iter_mut() {
            for (_, other_collision_shape) in other_collidables.collision_shapes.iter_mut() {
    
                if collision_shape.uuid == other_collision_shape.uuid {
                    continue;
                }
    
                if !can_collide(collision_shape, other_collision_shape) {
                    continue;
                }
    
                let is_colliding = collide(
                    transform.translation,
                    collision_shape.size(),
                    other_transform.translation,
                    other_collision_shape.size()
                );
                
                match is_colliding {
                    None => {
                        collision_shape.collisions.remove(&other_entity);
                        other_collision_shape.collisions.remove(&entity);
                    },
                    Some(_collision_side) => {
                        collision_shape.collisions.insert(other_entity);
                        other_collision_shape.collisions.insert(entity);
                    }
                }
    
            }
        }
    }
}

fn can_collide(shape_a: &CollisionShape, shape_b: &CollisionShape) -> bool {
    return (shape_a.mask as i32 & shape_b.collides_with != 0) &&
        (shape_b.mask as i32 & shape_a.collides_with != 0);
}

pub fn tick_collision_shapes(
    delta_time: Res<Time>,
    mut collidables_query: Query<&mut Collidables>
) {
    let mut to_remove = Vec::new();

    for mut collidables in collidables_query.iter_mut(){
        for (uuid, shape) in  collidables.collision_shapes.iter_mut() {
            shape.timer.tick(delta_time.delta());

            if shape.timer.repeating() {
                continue;
            }

            if shape.timer.finished() {
                to_remove.push(uuid.clone());
            }
        }

        for uuid in &to_remove {
            collidables.collision_shapes.remove(&uuid);
        }
    }
}