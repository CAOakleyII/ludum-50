use bevy::prelude::*;
mod systems;
mod components;

use crate::systems::{player_spawner, animate_sprites};


fn main() {
    App::new()
    .add_startup_system(player_spawner)
    .add_system(animate_sprites)
    .add_plugins(DefaultPlugins).run();
}