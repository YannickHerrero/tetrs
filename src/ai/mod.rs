pub mod difficulty;
pub mod evaluator;
pub mod placement;

use std::time::Duration;

use rand::Rng;

use self::difficulty::AiDifficulty;
use self::placement::{find_best_placement, Placement};
use crate::game::{GameAction, GameState};

/// AI player that controls a Tetris board.
#[derive(Debug, Clone)]
pub struct AiPlayer {
    pub difficulty: AiDifficulty,
    /// Current target placement.
    target: Option<Placement>,
    /// Think timer (delay before AI starts moving).
    think_timer: Duration,
    /// Move accumulator for movement speed.
    move_accumulator: Duration,
    /// Whether the AI has started moving the current piece.
    moving: bool,
    /// Track previous attack sent to detect new attacks.
    last_attack_sent: u32,
}

impl AiPlayer {
    pub fn new(difficulty: AiDifficulty) -> Self {
        Self {
            difficulty,
            target: None,
            think_timer: Duration::ZERO,
            move_accumulator: Duration::ZERO,
            moving: false,
            last_attack_sent: 0,
        }
    }

    /// Think and return actions for this frame.
    pub fn think(&mut self, game: &GameState, dt: Duration) -> Vec<GameAction> {
        if game.game_over || game.current_piece.is_none() {
            self.target = None;
            return Vec::new();
        }

        let piece = game.current_piece.as_ref().unwrap();

        // If we don't have a target, find one
        if self.target.is_none() {
            let weights = self.difficulty.weights();
            let mut best = find_best_placement(
                &game.board,
                piece.piece_type,
                game.hold.piece,
                &weights,
                self.difficulty.uses_hold(),
            );

            // Error chance: sometimes pick a worse placement
            if best.is_some() {
                let mut rng = rand::thread_rng();
                if rng.gen::<f64>() < self.difficulty.error_rate() {
                    let placements = placement::generate_placements(
                        &game.board,
                        piece.piece_type,
                        &weights,
                        false,
                    );
                    if placements.len() > 1 {
                        let idx = rng.gen_range(1..placements.len().min(5));
                        best = Some(placements[idx].clone());
                    }
                }
            }

            self.target = best;
            self.think_timer = self.difficulty.think_time();
            self.moving = false;
            self.move_accumulator = Duration::ZERO;
        }

        // Think delay
        if self.think_timer > Duration::ZERO {
            self.think_timer = self.think_timer.saturating_sub(dt);
            return Vec::new();
        }

        let target = match &self.target {
            Some(t) => t.clone(),
            None => return vec![GameAction::HardDrop],
        };

        let mut actions = Vec::new();

        // Check if we need to hold
        if target.use_hold && !game.hold.used_this_turn {
            self.target = None; // Will re-evaluate after hold
            return vec![GameAction::Hold];
        }

        // Calculate movement speed
        let move_interval = Duration::from_secs_f64(1.0 / self.difficulty.move_speed());
        self.move_accumulator += dt;

        if self.move_accumulator < move_interval {
            return Vec::new();
        }
        self.move_accumulator -= move_interval;

        // First: rotate to target rotation
        if piece.rotation != target.rotation {
            let target_rot = target.rotation;
            let current_rot = piece.rotation;

            // Determine shortest rotation direction
            let cw_dist = ((target_rot.index() as i32 - current_rot.index() as i32) + 4) % 4;
            let ccw_dist = ((current_rot.index() as i32 - target_rot.index() as i32) + 4) % 4;

            if cw_dist == 2 {
                actions.push(GameAction::Rotate180);
            } else if cw_dist <= ccw_dist {
                actions.push(GameAction::RotateCW);
            } else {
                actions.push(GameAction::RotateCCW);
            }
            return actions;
        }

        // Then: move horizontally
        if piece.x != target.x {
            if piece.x < target.x {
                actions.push(GameAction::MoveRight);
            } else {
                actions.push(GameAction::MoveLeft);
            }
            return actions;
        }

        // At target position: hard drop
        actions.push(GameAction::HardDrop);
        self.target = None;

        actions
    }

    /// Get the attack damage sent by the AI since last check.
    pub fn check_attack(&mut self, game: &GameState) -> u32 {
        let current = game.stats.attack_sent;
        let delta = current.saturating_sub(self.last_attack_sent);
        self.last_attack_sent = current;
        delta
    }

    /// Reset AI state for new game.
    pub fn reset(&mut self) {
        self.target = None;
        self.think_timer = Duration::ZERO;
        self.move_accumulator = Duration::ZERO;
        self.moving = false;
        self.last_attack_sent = 0;
    }
}
