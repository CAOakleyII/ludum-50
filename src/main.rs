extern crate queues;
use bevy::prelude::*;
mod components;
mod resources;
mod systems;
mod utils;

use crate::systems::*;


fn main() {
    App::new()
    .add_startup_system(player_spawner)
    .add_startup_system(insert_ai_resources)
    .add_system(animate_sprites)
    .add_system(player_input)
    .add_system(player_aim)
    .add_system(process_movement)
    .add_system(determine_movement_state)
    .add_system(process_state_queues)
    .add_system(animate_player_states)
    .add_system(player_combat_input)
    .add_plugins(DefaultPlugins).run();
}