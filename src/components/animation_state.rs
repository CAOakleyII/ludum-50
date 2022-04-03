use std::collections::{HashSet};

use bevy::prelude::{Component};

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
            current_state: State { kind: StateKind::Idle, interruptable: true },
            next_states: HashSet::new(),
            new_state: false
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct State {
    pub kind: StateKind,
    pub interruptable: bool,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum StateKind {
    Idle,
    Run
}

impl StateKind {
    pub fn to_string(&self) -> String {
        match self {
            Self::Idle  => "Idle".to_string(),
            Self::Run    => "Run".to_string(),
        }
    }
}
