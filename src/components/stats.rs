use bevy::prelude::{Component, Bundle};

#[derive(Component)]
pub struct Speed { 
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
}