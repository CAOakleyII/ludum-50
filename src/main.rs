use bevy::prelude::*;
mod systems;
mod components;

use crate::systems::{player_spawner, animate_sprites, player_input, process_movement};


fn main() {
    App::new()
    .add_startup_system(player_spawner)
    .add_system(animate_sprites)
    .add_system(player_input)
    .add_system(process_movement)
    .add_plugins(DefaultPlugins).run();
}