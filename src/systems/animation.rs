use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::{Path, RectangleOrigin, ShapePath, GeometryBuilder, FillMode, StrokeMode, DrawMode}, shapes, render::Shape};

use crate::components::{Stateful, HealthBar, CurrentHealth, MaxHealth, CollisionShape};

pub fn animate_sprites(
    delta_time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut Stateful, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>
) {
    for (mut timer, mut stateful, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(delta_time.delta());

        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            let index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as usize;
            sprite.index = index;

            if !stateful.current_state.should_loop && 
                index == texture_atlas.textures.len() - 1 {
                stateful.current_state.running = false;
            }
        }
    }
}

pub fn draw_health_bars(
    mut query: Query<(&mut Path, &Parent), With<HealthBar>>,
    parent_query: Query<(&CurrentHealth, &MaxHealth)>
) {
    for (mut path, parent) in query.iter_mut() {
        if let Ok((current_health, max_health)) = parent_query.get(parent.0) {
    
            let polygon = shapes::Rectangle { 
                origin: RectangleOrigin::TopLeft,
                extents: Vec2::new(40.0 * (current_health.value / max_health.value), 3.0)
            };
    
            *path = ShapePath::build_as(&polygon);
        }

    }
}

#[derive(Component)]
pub struct DebugShape;

pub fn draw_debug_hitboxes(
    mut commands: Commands,
    remove_query: Query<Entity, With<DebugShape>>,
    query: Query<(&CollisionShape, &Transform)>
) {
    for entity in remove_query.iter() {
        commands.entity(entity).despawn()
    }

    for (shape, transform) in query.iter() {
        let hitbox = shapes::Rectangle { 
            origin: RectangleOrigin::Center,
            extents: Vec2::new(shape.width, shape.height)
        };

        commands.spawn_bundle(GeometryBuilder::build_as(
            &hitbox,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::Rgba { red: (1.0), green: (0.0), blue: (0.0), alpha: (0.2) }),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            transform.clone()
        )).insert(DebugShape);
    }
}