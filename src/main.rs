extern crate queues;
use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
mod components;
mod resources;
mod systems;
mod utils;

use crate::systems::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(ShapePlugin)
    .add_startup_system(build_map)
    .add_startup_system(player_spawner)
    .add_startup_system(insert_ai_resources)
    .add_system(draw_debug_hitboxes)
    .add_system(animate_sprites)
    .add_system(draw_health_bars)
    .add_system(player_input)
    .add_system(player_combat_input)
    .add_system(player_jump_input)
    .add_system(player_aim)
    .add_system(process_movement)
    .add_system(process_jumping)
    .add_system(apply_gravity)
    .add_system(ground_collision)
    .add_system(determine_movement_state)
    .add_system(process_state_queues)
    .add_system(animate_player_states)
    .add_system(process_player_combat_states)
    .add_system(process_collisions)
    .add_system(tick_collision_shapes)
    .add_system(player_melee_attack_collision)
    .add_system(damage_entity)
    .add_system(shrink_map)
    .add_system(draw_map)
    .add_system(destroy_ai)
    .run();
}