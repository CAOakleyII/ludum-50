use bevy::{prelude::{Commands, Color, Transform}, math::Vec2};
use bevy_prototype_lyon::{shapes, prelude::{RectangleOrigin, GeometryBuilder, DrawMode, FillMode, StrokeMode}};

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
        Transform::from_xyz(0.0, -200.0, 0.0),
    ))
    .insert(floor_hitbox)
    .insert(Ground);
}