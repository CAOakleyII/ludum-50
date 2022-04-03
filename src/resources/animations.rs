use std::collections::HashMap;

use bevy::{prelude::Handle, sprite::TextureAtlas};

use crate::components::{StateKind, DirectionName};

pub struct PlayerAnimations { 
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
    pub animation_map: HashMap<StateKind, HashMap<DirectionName, Handle<TextureAtlas>>>

}