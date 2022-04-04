use std::collections::HashSet;
use bevy::{prelude::Component, math::Vec3, core::Timer};
use strum::{EnumIter, EnumString, Display};

#[derive(Component, Clone)]
pub struct Stateful {
    pub current_state: State,
    pub next_states: HashSet<State>,
    pub new_state: bool
}

impl Default for Stateful {
    fn default() -> Self {
        Self {
            current_state: State { 
                kind: StateKind::Idle,
                interruptable: true,
                should_loop: true,
                running: true,
                should_root: false
            },
            next_states: HashSet::new(),
            new_state: false
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct State {
    pub kind: StateKind,
    pub interruptable: bool,
    pub should_loop: bool,
    pub running: bool,
    pub should_root: bool
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, EnumIter, EnumString, Display)]
pub enum StateKind {
    Idle,
    Run,
    Jump,
    Fall,
    MeleeAttack,
    ChargeBow,
    ReleaseBow,
    ChargeMelee
}

impl StateKind {
    // Frame Data { Columns, Rows, TotalFrames }
    pub fn player_frame_data(self) -> Vec3{
        match self {
            Self::Idle => Vec3::new(5.0, 1.0, 5.0),
            Self::Run => Vec3::new(6.0, 1.0, 6.0),
            Self::Jump => Vec3::new(4.0, 1.0, 4.0),
            Self::Fall => Vec3::new(4.0, 1.0, 4.0),
            Self::MeleeAttack => Vec3::new(27.0, 1.0, 27.0),
            Self::ChargeBow => Vec3::new(5.0, 1.0, 5.0),
            Self::ReleaseBow => Vec3::new(6.0, 1.0, 6.0),
            _ => Vec3::ZERO
        }
    }

    pub fn ball_chain_bot_frame_data(self) -> Vec3 {
        match self {
            Self::Idle => Vec3::new(5.0, 1.0, 5.0),
            Self::Run => Vec3::new(8.0, 1.0, 8.0),
            Self::MeleeAttack => Vec3::new(8.0, 1.0, 8.0),
            Self::ChargeMelee => Vec3::new(4.0, 1.0, 4.0),
            _ => Vec3::ZERO
        }
    }
}


#[derive(Component)]
pub struct Damaged(pub f32);

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct Ground(pub f32);

#[derive(Component)]
pub struct Rooted;

#[derive(Component)]
pub struct Jumping {
    pub force: f32,
    pub timer: Timer,
    pub float_timer: Timer,
}
