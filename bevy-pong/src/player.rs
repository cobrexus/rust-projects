use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    Player1,
    Player2,
}
