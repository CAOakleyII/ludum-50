use bevy::{prelude::{Commands, Color, Transform, Res, Query}, math::Vec2, core::{Time, Timer}};
use bevy_prototype_lyon::{shapes, prelude::{RectangleOrigin, GeometryBuilder, DrawMode, FillMode, StrokeMode, Path, ShapePath}};

use crate::components::{CollisionShape, CollisionMasks, Ground};

pub fn build_map (
    mut commands: Commands
) { 

    let floor_hitbox = CollisionShape {
        width: 500.0,
        height: 50.0,
        mask: CollisionMasks::Ground,
        collides_with: CollisionMasks::Player as i32 | CollisionMasks::AI as i32,
        ..Default::default()
    };
    let ground_shape = shapes::Rectangle { 
        origin: RectangleOrigin::Center,
        extents: Vec2::new(500.0, 50.0)
    };
    commands.spawn_bundle(GeometryBuilder::build_as(
        &ground_shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::BLACK),
            outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        },
        Transform::from_xyz(0.0, -175.0, 0.0),
    ))
    .insert(Timer::from_seconds(15.0, true))
    .insert(floor_hitbox)
    .insert(Ground(500.0));
}

pub fn shrink_map (
    delta_time: Res<Time>,
    mut query: Query<(&mut Ground, &mut Timer)>,
) {

    for (mut ground, mut timer) in query.iter_mut() {
        timer.tick(delta_time.delta());
        if timer.just_finished() {
            ground.0 -= 100.0;
        }
    }

}

pub fn draw_map (
    mut query: Query<(&mut Path, &mut CollisionShape, &Ground)>
) {
    for (mut path, mut collision_shape, ground) in query.iter_mut() {
        let ground_shape = shapes::Rectangle { 
            origin: RectangleOrigin::Center,
            extents: Vec2::new(ground.0, 50.0)
        };

        *path = ShapePath::build_as(&ground_shape);
        collision_shape.width = ground.0;
    }
}