use bevy::{prelude::Component, math::Vec3};

#[derive(Component)]
pub struct Velocity {
    pub vector: Vec3
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            vector: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}