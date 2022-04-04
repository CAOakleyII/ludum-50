use std::collections::HashSet;

use bevy::{prelude::{Entity, Component}, core::Timer};

#[derive(Component, Clone)]
pub struct MeleeAttack {
    pub damage: f32,
    pub damage_table: HashSet<Entity>,
    pub width: f32,
    pub height: f32,
    pub timer: Timer,
}

impl Default for MeleeAttack {
    fn default() -> Self {
        Self {
            damage: 20.0,
            damage_table: HashSet::new(),
            width: 20.0,
            height: 20.0,
            timer: Timer::from_seconds(1.0, false)
        }
    }
}