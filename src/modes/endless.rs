use crate::game::GameState;
use crate::modes::GameMode;
use crate::ui::screens::game_over::GameResult;
use crate::ui::widgets::sidebar::format_number;

/// Endless marathon mode.
pub struct EndlessMode;

impl EndlessMode {
    pub fn new() -> Self {
        Self
    }
}

impl GameMode for EndlessMode {
    fn name(&self) -> &str {
        "ENDLESS"
    }

    fn on_start(&mut self, _game: &mut GameState) {
        // Nothing special
    }

    fn on_update(&mut self, _game: &mut GameState) {
        // Nothing per-tick
    }

    fn check_complete(&self, game: &GameState) -> Option<GameResult> {
        if game.game_over {
            Some(GameResult {
                mode_name: "Endless Marathon".to_string(),
                primary_label: "SCORE".to_string(),
                primary_value: format_number(game.scoring.score),
                is_new_high_score: false,
                stats: game.stats.clone(),
                won: None,
            })
        } else {
            None
        }
    }

    fn info_text(&self, _game: &GameState) -> Option<String> {
        None
    }
}
