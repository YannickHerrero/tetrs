pub mod endless;
pub mod sprint;
pub mod versus;

use crate::game::GameState;
use crate::ui::screens::game_over::GameResult;

/// Trait for game modes.
pub trait GameMode {
    /// Mode display name.
    fn name(&self) -> &str;

    /// Called when the game starts.
    fn on_start(&mut self, game: &mut GameState);

    /// Called each update tick. Can modify game state or check completion.
    fn on_update(&mut self, game: &mut GameState);

    /// Check if the mode is complete. Returns result if so.
    fn check_complete(&self, game: &GameState) -> Option<GameResult>;

    /// Get mode-specific info text to display (e.g., "Lines: 12/40").
    fn info_text(&self, game: &GameState) -> Option<String>;
}
