use std::time::Duration;

use crate::ai::difficulty::AiDifficulty;
use crate::ai::AiPlayer;
use crate::game::GameState;
use crate::modes::GameMode;
use crate::ui::screens::game_over::GameResult;

/// Versus AI mode.
pub struct VersusMode {
    pub ai: AiPlayer,
    pub ai_game: GameState,
    pub difficulty: AiDifficulty,
}

impl VersusMode {
    pub fn new(difficulty: AiDifficulty) -> Self {
        Self {
            ai: AiPlayer::new(difficulty),
            ai_game: GameState::new(),
            difficulty,
        }
    }

    /// Update the AI game state. Should be called each frame.
    pub fn update_ai(&mut self, dt: Duration) {
        if self.ai_game.game_over {
            return;
        }

        // AI thinks and acts
        let actions = self.ai.think(&self.ai_game, dt);
        for action in actions {
            self.ai_game.handle_action(action);
        }

        let result = self.ai_game.update(dt);
        if result.attack > 0 {
            // AI sends attack to player (handled in app)
        }
    }

    /// Get the AI's attack damage from last update (drain events).
    pub fn drain_ai_attack(&mut self) -> u32 {
        let _events = self.ai_game.drain_events();
        self.ai_game.stats.attack_sent
    }
}

impl GameMode for VersusMode {
    fn name(&self) -> &str {
        "VERSUS"
    }

    fn on_start(&mut self, _game: &mut GameState) {
        self.ai_game.start();
        self.ai = AiPlayer::new(self.difficulty);
    }

    fn on_update(&mut self, _game: &mut GameState) {
        // AI update happens separately in the app loop
    }

    fn check_complete(&self, game: &GameState) -> Option<GameResult> {
        let player_dead = game.game_over;
        let ai_dead = self.ai_game.game_over;

        if player_dead || ai_dead {
            let won = ai_dead && !player_dead;
            Some(GameResult {
                mode_name: format!("Versus AI ({})", self.difficulty.name()),
                primary_label: if won { "VICTORY" } else { "DEFEAT" }.to_string(),
                primary_value: format!(
                    "ATK: {} | DMG: {}",
                    game.stats.attack_sent, game.stats.garbage_received
                ),
                is_new_high_score: false,
                stats: game.stats.clone(),
                won: Some(won),
            })
        } else {
            None
        }
    }

    fn info_text(&self, game: &GameState) -> Option<String> {
        Some(format!(
            "ATK:{} RCV:{}",
            game.stats.attack_sent, game.stats.garbage_received
        ))
    }
}
