use bevy::{prelude::Component, math::Vec3};

#[derive(Component)]
pub struct Direction {
    pub angle: f32,
    pub name: DirectionName,
    pub new_direction: bool
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum DirectionName {
    Left,
    Up,
    Right,
    Down
}

#[derive(Component)]
pub struct Aim {
    pub vector: Vec3
}

impl Default for Aim {
    fn default() -> Self {
        Self {
            vector: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}