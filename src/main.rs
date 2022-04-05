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
    .add_system(ball_chain_bot_spawner)
    // Set Animations
    .add_system(draw_debug_hitboxes)
    .add_system(animate_sprites)
    .add_system(draw_health_bars)
    // Set Player Input
    .add_system(player_input)
    .add_system(player_combat_input)
    .add_system(player_jump_input)
    .add_system(player_dash_input)
    .add_system(player_dash_attack_input)
    .add_system(player_aim)
    // Set Movement
    .add_system(process_movement)
    .add_system(process_jumping)
    .add_system(process_dashing)
    .add_system(apply_gravity)
    // Map
    .add_system(ground_collision)
    .add_system(shrink_map)
    .add_system(draw_map)
    .add_system(determine_movement_state)
    // Set States
    .add_system(process_state_queues)
    .add_system(animate_player_states)
    .add_system(process_player_combat_states)
    // Set Collisions
    .add_system(process_collisions)
    .add_system(tick_collision_shapes)
    // Set Combat
    .add_system(player_melee_attack)
    .add_system(player_melee_attack_collision)
    .add_system(damage_entity)
    .add_system(destroy_ai)
    .run();
}