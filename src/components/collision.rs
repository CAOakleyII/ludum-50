use std::collections::HashSet;

use bevy::{prelude::{Component, Entity}, math::Vec2, core::Timer };
use strum::Display;
use uuid::Uuid;

#[derive(Component)]
pub struct CollisionShape {
    pub uuid: Uuid,
    pub width: f32,
    pub height: f32,
    pub mask: CollisionMasks,
    pub collides_with: i32,
    pub collisions: HashSet<Entity>,
    pub collisions_just_ended: HashSet<Entity>,
    pub timer: Timer
}

impl Default for CollisionShape {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            width: 20.0,
            height: 20.0,
            mask: CollisionMasks::Player,
            collides_with: 0,
            collisions: HashSet::new(),
            collisions_just_ended: HashSet::new(),
            timer: Timer::from_seconds(1.0, true)
        }
    }
}

impl CollisionShape {
    pub fn size(&self) -> Vec2 {
        return Vec2::new(self.width, self.height)
    }
}

#[derive(Clone, Copy, Display)]
pub enum CollisionMasks {
    Player = 0x1,
    AI = 0x2,
    PlayerAttack = 0x4,
    AIAttack = 0x8,
    Ground = 0x16
}