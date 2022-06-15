#![cfg_attr(debug_assertions, allow(dead_code))]

/// Represents the current screens being displayed by the game.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Title,
    LevelSelect,
    Level,
    GameComplete,
    Settings,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Title
    }
}

/// Represents the current level that is being played.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum LevelState {
    Level1,
    Level2,
}

impl LevelState {
    /// Returns the next level. If the current level is the last, returns
    /// [`None`].
    pub fn next_level(&self) -> Option<Self> {
        use LevelState::*;

        match self {
            Level1 => Some(Level2),
            _ => None,
        }
    }
}

impl Default for LevelState {
    fn default() -> Self {
        Self::Level1
    }
}
