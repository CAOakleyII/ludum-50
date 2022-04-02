use bevy::prelude::Component;

#[derive(Component)]
pub struct AimVector {
    pub x: f32,
    pub y: f32
}

impl Default for AimVector {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}