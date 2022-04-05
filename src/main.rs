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
    // Map
    .add_system_set(
        SystemSet::new()
            .label("map_rules")
            .before("despawners")
            .before("player_input")
            .with_system(ground_collision)
            .with_system(shrink_map)
            .with_system(draw_map)
    )
    // Set Player Input
    .add_system_set(
        SystemSet::new()
            .label("player_input")
            .before("despawners")
            .before("combat")
            .with_system(player_input)
            .with_system(player_combat_input)
            .with_system(player_jump_input)
            .with_system(player_dash_input)
            .with_system(player_dash_attack_input)
            .with_system(player_aim)
    )
    // Set States
    .add_system_set(
        SystemSet::new()
            .label("process_states")
            .before("despawners")
            .before("combat")
            .with_system(process_state_queues)
            .with_system(process_combat_states)
            .with_system(determine_movement_state)
    )
    // Set Movement
    .add_system_set(
        SystemSet::new()
            .label("movement_systems")
            .before("despawners")
            .before("combat")
            .after("process_states")
            .with_system(process_movement)
            .with_system(process_jumping)
            .with_system(process_dashing)
            .with_system(apply_gravity)
            .with_system(apply_roots)
    )
    // Set Animations
    .add_system_set(
        SystemSet::new()
            .label("animations")
            .before("despawners")
            .after("process_states")
            .with_system(draw_debug_hitboxes)
            .with_system(animate_sprites)
            .with_system(draw_health_bars)
            .with_system(animate_player_states)
            .with_system(animate_ball_chain_bot_states)
            .with_system(draw_hit)
    )
    // Set Combat
    .add_system_set(
        SystemSet::new()
            .label("combat")
            .before("despawners")
            .with_system(process_melee_attacks)
            .with_system(player_melee_attack_collision)
            .with_system(damage_entity)
            .with_system(destroy_ai)
    )
    // Set Collisions
    .add_system_set(
        SystemSet::new()
            .label("collision")
            .before("despawners")
            .after("combat")
            .with_system(process_collisions)
            .with_system(tick_collision_shapes)
    )
    // Set AI
    .add_system_set(
        SystemSet::new()
            .label("ai_processing")
            .before("despawners")
            .with_system(ball_chain_bot_spawner)
            .with_system(chase_player)
            .with_system(face_player)
            .with_system(determine_ai_in_attack_range)
            .with_system(attack_player)
            .with_system(tick_ai_cool_downs)
    )
    .add_system_set(
        SystemSet::new()
            .label("despawners")
            .with_system(process_death)
    )
    .run();
}