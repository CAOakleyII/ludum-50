use std::collections::HashMap;

use bevy::prelude::*;
use bevy::sprite::{TextureAtlas, SpriteSheetBundle};
use bevy::math::Vec2;
use crate::components::{Player, Velocity, Aim, Speed, Direction, EntityType, DirectionName, StateKind, Stateful};
use crate::resources::PlayerAnimations;

pub fn player_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let mut animation_map: HashMap<StateKind, HashMap<DirectionName, Handle<TextureAtlas>>>  = HashMap::new();

    // Idle Animations
    let idle_down_image_handle = asset_server.load("player/down/idle.png");
    let idle_down_texture_atlas = TextureAtlas::from_grid(idle_down_image_handle, Vec2::new(64.0, 64.0), 5, 1);
    let idle_down_texture_atlas_handle = texture_atlases.add(idle_down_texture_atlas); 

    let idle_up_image_handle = asset_server.load("player/up/idle.png");
    let idle_up_texture_atlas = TextureAtlas::from_grid(idle_up_image_handle, Vec2::new(64.0, 64.0), 5, 1);
    let idle_up_texture_atlas_handle = texture_atlases.add(idle_up_texture_atlas); 

    let idle_right_image_handle = asset_server.load("player/side/idle.png");
    let idle_right_texture_atlas = TextureAtlas::from_grid(idle_right_image_handle, Vec2::new(64.0, 64.0), 5, 1);
    let idle_right_texture_atlas_handle = texture_atlases.add(idle_right_texture_atlas); 

    animation_map.insert(
        StateKind::Idle,
        HashMap::from([
            (DirectionName::Down, idle_down_texture_atlas_handle.clone()),
            (DirectionName::Up, idle_up_texture_atlas_handle),
            (DirectionName::Left, idle_right_texture_atlas_handle.clone()),
            (DirectionName::Right, idle_right_texture_atlas_handle)
        ])
    );

    // Run Animations
    let run_down_image_handle = asset_server.load("player/down/run.png");
    let run_down_texture_atlas = TextureAtlas::from_grid(run_down_image_handle, Vec2::new(64.0, 64.0), 6, 1);
    let run_down_texture_atlas_handle = texture_atlases.add(run_down_texture_atlas); 

    let run_up_image_handle = asset_server.load("player/up/run.png");
    let run_up_texture_atlas = TextureAtlas::from_grid(run_up_image_handle, Vec2::new(64.0, 64.0), 6, 1);
    let run_up_texture_atlas_handle = texture_atlases.add(run_up_texture_atlas); 

    let run_right_image_handle = asset_server.load("player/side/run.png");
    let run_right_texture_atlas = TextureAtlas::from_grid(run_right_image_handle, Vec2::new(64.0, 64.0), 6, 1);
    let run_right_texture_atlas_handle = texture_atlases.add(run_right_texture_atlas); 

    animation_map.insert(
        StateKind::Run,
        HashMap::from([
            (DirectionName::Down, run_down_texture_atlas_handle.clone()),
            (DirectionName::Up, run_up_texture_atlas_handle),
            (DirectionName::Left, run_right_texture_atlas_handle.clone()),
            (DirectionName::Right, run_right_texture_atlas_handle)
        ])
    );

    commands.insert_resource(PlayerAnimations {
        animation_map
    });

    // create camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.5;
    commands.spawn_bundle(camera);

    // create player
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