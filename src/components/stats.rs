use bevy::prelude::{Component, Bundle};

#[derive(Component)]
pub struct Speed { 
    pub value: f32 
}
#[derive(Component)]
pub struct JumpHeight { 
    pub value: f32 
}

#[derive(Component)]
pub struct DashDistance { 
    pub value: f32 
}

#[derive(Component)]
pub struct Gravity {
    pub value: f32
}

#[derive(Component)]
pub struct CurrentHealth {
    pub value: f32
}

#[derive(Component)]
pub struct MaxHealth{
    pub value: f32
}

#[derive(Bundle)]
pub struct StatsBundle {
    pub speed: Speed,
    pub jump_height: JumpHeight,
    pub current_health: CurrentHealth,
    pub max_health: MaxHealth,
    pub dash_distance: DashDistance,
    pub gravity: Gravity,
}

impl Default for StatsBundle {
    fn default() -> Self {
        Self {
            speed: Speed { value: 100.0 },
            jump_height: JumpHeight { value: 375.0 },
            current_health: CurrentHealth { value: 100.0 },
            max_health: MaxHealth { value: 100.0 },
            dash_distance: DashDistance { value: 75.0 },
            gravity: Gravity { value: 175.0 }
        }
    }
}