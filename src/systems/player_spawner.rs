use std::collections::{HashMap};

use bevy::prelude::*;
use bevy::sprite::{TextureAtlas, SpriteSheetBundle};
use bevy::math::Vec2;
use bevy_prototype_lyon::prelude::{RectangleOrigin, GeometryBuilder, DrawMode, FillMode, StrokeMode};
use bevy_prototype_lyon::shapes;
use strum::IntoEnumIterator;
use crate::components::{Player, Velocity, Aim, Direction, DirectionName, StateKind, Stateful, CollisionShape, CollisionMasks, MeleeAttack, StatsBundle, HealthBar};
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
                                                 state.player_frame_data().x as usize,
                                                 state.player_frame_data().y as usize
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
    camera.orthographic_projection.scale = 0.38;
    commands.spawn_bundle(camera);

    // create player
    // Idle Animations
    let idle_down_image_handle = asset_server.load("player/right/idle.png");
    let idle_down_texture_atlas = TextureAtlas::from_grid(idle_down_image_handle, Vec2::new(64.0, 64.0), 5, 1);
    let idle_down_texture_atlas_handle = texture_atlases.add(idle_down_texture_atlas);

    let player_hitbox = CollisionShape{
        collides_with: CollisionMasks::AI as i32 | CollisionMasks::AIAttack as i32 | CollisionMasks::Ground as i32,
        ..Default::default() 
    };

    let player = commands.spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: idle_down_texture_atlas_handle,
            transform: Transform::from_xyz(0.0,-100.0,1.0),
            ..Default::default()
        })
        .insert(player_hitbox)
        .insert(Stateful { ..Default::default() })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Player)
        .insert(Direction { angle: 0.0, name: crate::components::DirectionName::Right, new_direction: false, flip_x: 1.0 })
        .insert(Velocity { ..Default::default() })
        .insert_bundle(StatsBundle { ..Default::default() })
        .insert(Aim { ..Default::default() })
        .insert(MeleeAttack { 
            width: 17.0,
            height: 30.0,
            ..Default::default() })
        .id();

    let health_bar = shapes::Rectangle { 
        origin: RectangleOrigin::TopLeft,
        extents: Vec2::new(20.0, 3.0)
    };

    let health_bar_entity = commands.spawn_bundle(GeometryBuilder::build_as(
        &health_bar,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::RED),
            outline_mode: StrokeMode::new(Color::BLACK, 0.0),
        },
        Transform::from_xyz(-10.0, 14.0, 0.0),
    ))
    .insert(HealthBar)
    .insert(Parent(player))
    .id();

    let health_bar_outline = shapes::Rectangle { 
        origin: RectangleOrigin::TopLeft,
        extents: Vec2::new(21.0, 4.0)
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &health_bar_outline,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::rgba(1.0, 1.0, 1.0, 1.0)),
            outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        },
        Transform::from_xyz(-0.5, 0.5, 0.0),
    ))
    .insert(Parent(health_bar_entity));
}