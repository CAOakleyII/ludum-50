use std::collections::HashMap;

use bevy::prelude::*;
use bevy::sprite::{TextureAtlas, SpriteSheetBundle};
use bevy::math::Vec2;
use strum::IntoEnumIterator;
use crate::components::{Player, Velocity, Aim, Speed, Direction, EntityType, DirectionName, StateKind, Stateful};
use crate::resources::PlayerAnimations;

pub fn player_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let mut animation_map: HashMap<StateKind, HashMap<DirectionName, Handle<TextureAtlas>>>  = HashMap::new();

    // for state
        // for direction (left|right -> side)
            // define path | as formatted file
            // 
    for state in StateKind::iter() {
        let mut hash_map = HashMap::new();
        for direction in DirectionName::iter() {
            let path = format!("player/{}/{}.png", direction, state);
            let image_handle: Handle<Image> = asset_server.load(&path);
            let texture_atlas = TextureAtlas::from_grid(
                                                image_handle,
                                                Vec2::new(64.0, 64.0),
                                                 state.frame_data().x as usize,
                                                 state.frame_data().y as usize
                                            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas); 

            hash_map.insert(direction, texture_atlas_handle);
        }
        animation_map.insert(
            state,
            hash_map
        );
    }

    commands.insert_resource(PlayerAnimations {
        animation_map
    });

    // create camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.5;
    commands.spawn_bundle(camera);


    // create player
    // Idle Animations
    let idle_down_image_handle = asset_server.load("player/down/idle.png");
    let idle_down_texture_atlas = TextureAtlas::from_grid(idle_down_image_handle, Vec2::new(64.0, 64.0), 5, 1);
    let idle_down_texture_atlas_handle = texture_atlases.add(idle_down_texture_atlas);

    commands.spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: idle_down_texture_atlas_handle,
            transform: Transform::from_xyz(0.0,0.0,0.0),
            ..Default::default()
        })
        .insert(Stateful { ..Default::default() })
        .insert(Timer::from_seconds(0.1, true))
        .insert(EntityType { name: "player".to_string() })
        .insert(Player)
        .insert(Direction { angle: 0.0, name: crate::components::DirectionName::Down, new_direction: false })
        .insert(Velocity { ..Default::default() })
        .insert(Speed { value: 100.0 })
        .insert(Aim { ..Default::default() });
}