use bevy::{prelude::*, ecs::system::Command};
use bevy_prototype_lyon::{prelude::{Path, RectangleOrigin, ShapePath, GeometryBuilder, FillMode, StrokeMode, DrawMode}, shapes};

use crate::{components::{Stateful, HealthBar, CurrentHealth, MaxHealth, CollisionShape, Player, DirectionName, Direction, BallChainBot, Hit}, resources::{PlayerAnimations, BallChainBotAnimations}};

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
            let health_percent = (current_health.value / max_health.value).clamp(0.0, 1.0);
            let polygon = shapes::Rectangle { 
                origin: RectangleOrigin::TopLeft,
                extents: Vec2::new((20.0 * health_percent).clamp(0.0, 20.0), 3.0)
            };
    
            *path = ShapePath::build_as(&polygon);
        }
    }
}

pub fn animate_player_states(
    player_animations: Res<PlayerAnimations>,
    mut query: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &Stateful, &mut Direction), With<Player>>
) {
    for (mut handle, mut sprite, stateful, mut direction) in query.iter_mut() {
        if !stateful.new_state && !direction.new_direction {
            continue
        }

        *handle = player_animations.animation_map.get(&stateful.current_state.kind).unwrap().get(&direction.name).unwrap().clone();

        sprite.index = 0 as usize;
        if &direction.name == &DirectionName::Left {
            sprite.flip_x = true;
            direction.flip_x = -1.0;
            // x = x * -1
        } else {
            sprite.flip_x = false;
            direction.flip_x = 1.0;
        }
    }
}

pub fn animate_ball_chain_bot_states(
    ai_animations: Res<BallChainBotAnimations>,
    mut query: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &Stateful, &mut Direction), With<BallChainBot>>
) {
    for (mut handle, mut sprite, stateful, mut direction) in query.iter_mut() {
        if !stateful.new_state && !direction.new_direction {
            continue
        }

        *handle = ai_animations.animation_map.get(&stateful.current_state.kind).unwrap().get(&direction.name).unwrap().clone();

        sprite.index = 0 as usize;
        if &direction.name == &DirectionName::Left {
            sprite.flip_x = true;
            direction.flip_x = -1.0;
            // x = x * -1
        } else {
            sprite.flip_x = false;
            direction.flip_x = 1.0;
        }
    }
}

pub fn draw_hit(
    mut commands: Commands,
    delta_time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Hit, Entity)>
) {
    for (mut sprite, mut hit, entity) in query.iter_mut() {
        if hit.0.finished() {
            return;
        }
        if sprite.color == Color::WHITE {
            sprite.color = Color::BLACK;
        } else {
            sprite.color = Color::WHITE;
        }

        hit.0.tick(delta_time.delta());

        if hit.0.just_finished() {
            sprite.color = Color::WHITE;            
            commands.entity(entity)
                .remove::<Hit>();
        }
    }
}

#[derive(Component)]
pub struct DebugShape;

pub fn draw_debug_hitboxes(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    remove_query: Query<Entity, With<DebugShape>>,
    query: Query<(&CollisionShape, &Transform)>
) {
    for entity in remove_query.iter() {
        commands.entity(entity).despawn()
    }

    if keyboard_input.pressed(KeyCode::LAlt) {
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
}