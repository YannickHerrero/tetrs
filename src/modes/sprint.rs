use crate::game::GameState;
use crate::modes::GameMode;
use crate::ui::screens::game_over::GameResult;

/// 40-line sprint mode.
pub struct SprintMode {
    pub target_lines: u32,
}

impl SprintMode {
    pub fn new() -> Self {
        Self { target_lines: 40 }
    }
}

impl GameMode for SprintMode {
    fn name(&self) -> &str {
        "SPRINT"
    }

    fn on_start(&mut self, _game: &mut GameState) {
        // Nothing special for sprint
    }

    fn on_update(&mut self, _game: &mut GameState) {
        // Nothing per-tick
    }

    fn check_complete(&self, game: &GameState) -> Option<GameResult> {
        if game.game_over {
            return Some(GameResult {
                mode_name: format!("{}-Line Sprint", self.target_lines),
                primary_label: "TIME".to_string(),
                primary_value: game.stats.format_time(),
                is_new_high_score: false, // Set by app
                stats: game.stats.clone(),
                won: None,
            });
        }

        if game.scoring.lines_cleared >= self.target_lines {
            return Some(GameResult {
                mode_name: format!("{}-Line Sprint", self.target_lines),
                primary_label: "TIME".to_string(),
                primary_value: game.stats.format_time(),
                is_new_high_score: false,
                stats: game.stats.clone(),
                won: None,
            });
        }

        None
    }

    fn info_text(&self, game: &GameState) -> Option<String> {
        let remaining = self.target_lines.saturating_sub(game.scoring.lines_cleared);
        Some(format!("{} lines left", remaining))
    }
}
