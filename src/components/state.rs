use std::collections::{HashSet};

use bevy::{prelude::{Component}, math::Vec2};
use strum::{EnumIter, EnumString, Display};

#[derive(Component)]
pub struct EntityType {
    pub name: String,
}

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
                running: true
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
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, EnumIter, EnumString, Display)]
pub enum StateKind {
    Idle,
    Run,
    MeleeAttack,
    ChargeBow,
    ReleaseBow
}

impl StateKind {
    pub fn frame_data(self) -> Vec2{
        match self {
            Self::Idle => Vec2::new(5.0, 1.0),
            Self::Run => Vec2::new(6.0, 1.0),
            Self::MeleeAttack => Vec2::new(27.0, 1.0),
            Self::ChargeBow => Vec2::new(5.0, 1.0),
            Self::ReleaseBow => Vec2::new(6.0, 1.0),
        }
    }
}
