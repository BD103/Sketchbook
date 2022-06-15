#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Title,
    LevelSelect,
    Level,
    GameComplete,
    Settings,
}
