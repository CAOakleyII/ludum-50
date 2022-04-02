use bevy::prelude::*;
use bevy::sprite::{TextureAtlas, SpriteSheetBundle};
use bevy::math::Vec2;
use crate::components::{Player, Velocity, AimVector, Speed};

pub fn player_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let texture_handle = asset_server.load("player/down/idle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas); 

    // create camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.5;
    commands.spawn_bundle(camera);

    // create player
    commands.spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0.0,0.0,0.0),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Player)
        .insert(Velocity { ..Default::default() })
        .insert(Speed { value: 100.0 })
        .insert(AimVector{ ..Default::default() });
}