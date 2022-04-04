use bevy::prelude::*;

use crate::components::Stateful;

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