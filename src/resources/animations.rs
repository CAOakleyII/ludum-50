use std::collections::HashMap;

use bevy::{prelude::Handle, sprite::TextureAtlas};

use crate::components::{StateKind, DirectionName};

/*
    Animation Map Structure
    -----------------------
    animation_map = { 
            Idle: {
                Down: Handle<Texture>,
                Up: Handle<Texture>,
                Side: Handle<Texture>
            },
            Run: {
                Down: Handle<Texture>,
                Up: Handle<Texture>,
                Side: Handle<Texture>
            },
            Attack: {
                Down: Handle<Texture>,
                Up: Handle<Texture>,
                Side: Handle<Texture>
            }
    }
*/
pub struct PlayerAnimations { 
    pub animation_map: HashMap<StateKind, HashMap<DirectionName, Handle<TextureAtlas>>>
}

pub struct BallChainBotAnimations {
    pub animation_map: HashMap<StateKind, HashMap<DirectionName, Handle<TextureAtlas>>>
}