use std::collections::HashMap;

use bevy::prelude::*;
use strum::IntoEnumIterator;
use crate::{components::*, resources::BallChainBotAnimations};

pub fn insert_ai_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let mut animation_map: HashMap<StateKind, HashMap<DirectionName, Handle<TextureAtlas>>>  = HashMap::new();

    for state in StateKind::iter() {
        let mut hash_map = HashMap::new();
        for direction in DirectionName::iter() {
            let path = format!("ball_chain_bot\\{}\\{}.png", direction, state);
            let image_handle: Handle<Image> = asset_server.load(&path);
            let texture_atlas = TextureAtlas::from_grid(
                                                image_handle,
                                                Vec2::new(192.0, 192.0),
                                                 state.ball_chain_bot_frame_data().x as usize,
                                                 state.ball_chain_bot_frame_data().y as usize
                                            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas); 

            hash_map.insert(direction, texture_atlas_handle);
        }
        animation_map.insert(
            state,
            hash_map
        );
    }

    commands.insert_resource(BallChainBotAnimations {
        animation_map
    });

    // Create AI To Test
    let idle_down_image_handle = asset_server.load("ball_chain_bot\\right\\idle.png");
    let idle_down_texture_atlas = TextureAtlas::from_grid(idle_down_image_handle, Vec2::new(192.0, 192.0), 5, 1);
    let idle_down_texture_atlas_handle = texture_atlases.add(idle_down_texture_atlas);

    let mut collision_shapes = HashMap::new();
    let ai_hitbox = CollisionShape{
        width: 25.0,
        height: 25.0,
        mask: CollisionMasks::AI,
        collides_with: CollisionMasks::Player as i32 | CollisionMasks::PlayerAttack as i32,
        ..Default::default() 
    };
    
    collision_shapes.insert(ai_hitbox.uuid, ai_hitbox);

    commands.spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: idle_down_texture_atlas_handle,
            transform: Transform::from_xyz(0.0,0.0,0.0),
            ..Default::default()
        })
        .insert(Collidables { collision_shapes })
        .insert(Stateful { ..Default::default() })
        .insert(Timer::from_seconds(0.1, true))
        .insert(AI)
        .insert(crate::components::Direction { angle: 0.0, name: crate::components::DirectionName::Right, new_direction: false })
        .insert(Velocity { ..Default::default() })
        .insert(Speed { value: 100.0 })
        .insert(Aim { ..Default::default() });
}