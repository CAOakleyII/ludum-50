use std::collections::HashSet;

use bevy::{prelude::{Entity, Component}, core::Timer, math::Vec3};

use super::CollisionMasks;
#[derive(Component, Clone)]
pub struct MeleeAttack {
    pub damage: f32,
    pub damage_table: HashSet<Entity>,
    pub width: f32,
    pub height: f32,
    pub knockback_force: f32,
    pub offset: Vec3,
    pub active_frame_length: f32,
    pub timer: Timer,
    pub full_attack_timer: Timer,
    pub cool_down_timer: Timer,
    pub character_id: Vec<Entity>,
    pub forward_step: f32,
    pub mask: CollisionMasks,
    pub collides_with: i32,
}

impl Default for MeleeAttack {
    fn default() -> Self {
        Self {
            damage: 20.0,
            damage_table: HashSet::new(),
            width: 20.0,
            height: 20.0,
            knockback_force: 100.0, 
            offset: Vec3::new(0.0, 0.0, 0.0),
            active_frame_length: 0.0,
            timer: Timer::from_seconds(1.0, true),
            full_attack_timer: Timer::from_seconds(1.0, false),
            cool_down_timer: Timer::from_seconds(5.0, true),
            character_id: Vec::new(),
            forward_step: 0.0,
            mask: CollisionMasks::PlayerAttack,
            collides_with: CollisionMasks::AI as i32
        }
    }
}