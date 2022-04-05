use bevy::{prelude::Component, math::Vec3};
use strum::{EnumIter, EnumString, Display};

#[derive(Component)]
pub struct Direction {
    pub angle: f32,
    pub name: DirectionName,
    pub new_direction: bool,
    pub flip_x: f32,
}

#[derive(Clone, Hash, PartialEq, Eq, EnumIter, EnumString, Display)]
pub enum DirectionName {
    Left,
    // Up,
    Right,
    // Down
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