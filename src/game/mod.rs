pub mod bag;
pub mod board;
pub mod clear;
pub mod garbage;
pub mod ghost;
pub mod gravity;
pub mod hold;
pub mod locking;
pub mod movement;
pub mod piece;
pub mod scoring;
pub mod srs;
pub mod stats;

use std::time::Duration;

use rand::rngs::ThreadRng;

use self::bag::Bag;
use self::board::{Board, VISIBLE_HEIGHT};
use self::clear::{ClearType, SpinType};
use self::garbage::GarbageQueue;
use self::ghost::ghost_y;
use self::gravity::Gravity;
use self::hold::Hold;
use self::locking::LockDelay;
use self::movement::{detect_spin, is_grounded};
use self::piece::{Piece, PieceType};
use self::scoring::Scoring;
use self::stats::Stats;

/// Actions the player can take.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    RotateCW,
    RotateCCW,
    Rotate180,
    Hold,
    SoftDropRelease,
}

/// Result of a game update tick.
#[derive(Debug, Clone)]
pub struct TickResult {
    pub lines_cleared: u32,
    pub clear_type: ClearType,
    pub is_perfect_clear: bool,
    pub score_gained: u64,
    pub attack: u32,
    pub piece_locked: bool,
    pub game_over: bool,
    pub hard_dropped: bool,
    pub spin_type: SpinType,
}

impl TickResult {
    fn none() -> Self {
        Self {
            lines_cleared: 0,
            clear_type: ClearType::None,
            is_perfect_clear: false,
            score_gained: 0,
            attack: 0,
            piece_locked: false,
            game_over: false,
            hard_dropped: false,
            spin_type: SpinType::None,
        }
    }
}

/// Visual effect event for the renderer.
#[derive(Debug, Clone)]
pub enum GameEvent {
    PieceLocked,
    LinesClear(Vec<usize>),
    HardDrop { cells: u32 },
    TSpin(SpinType),
    PerfectClear,
    Combo(u32),
    BackToBack(u32),
    GarbageReceived(u32),
    GameOver,
    LevelUp(u32),
}

/// Core game state for a single Tetris board.
#[derive(Debug, Clone)]
pub struct GameState {
    pub board: Board,
    pub current_piece: Option<Piece>,
    pub bag: Bag,
    pub hold: Hold,
    pub gravity: Gravity,
    pub lock_delay: LockDelay,
    pub scoring: Scoring,
    pub garbage: GarbageQueue,
    pub stats: Stats,
    pub rng: ThreadRng,
    pub game_over: bool,
    pub started: bool,

    // State tracking for spin detection
    pub last_was_rotation: bool,
    pub last_kick: Option<(i32, i32)>,

    // Visual state
    pub clearing_lines: Option<(Vec<usize>, Duration)>,
    pub events: Vec<GameEvent>,
    pub last_clear_type: Option<ClearType>,
    pub last_clear_time: Duration,
    pub action_text_timer: Duration,

    // Line clear animation duration
    clear_delay: Duration,
}

impl GameState {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let bag = Bag::new(&mut rng);
        Self {
            board: Board::new(),
            current_piece: None,
            bag,
            hold: Hold::new(),
            gravity: Gravity::new(),
            lock_delay: LockDelay::new(),
            scoring: Scoring::new(),
            garbage: GarbageQueue::new(),
            stats: Stats::new(),
            rng,
            game_over: false,
            started: false,
            last_was_rotation: false,
            last_kick: None,
            clearing_lines: None,
            events: Vec::new(),
            last_clear_type: None,
            last_clear_time: Duration::ZERO,
            action_text_timer: Duration::ZERO,
            clear_delay: Duration::from_millis(200),
        }
    }

    /// Start / reset the game.
    pub fn start(&mut self) {
        let mut rng = rand::thread_rng();
        self.board = Board::new();
        self.bag = Bag::new(&mut rng);
        self.hold.reset();
        self.gravity = Gravity::new();
        self.lock_delay.reset();
        self.scoring.reset();
        self.garbage.clear();
        self.stats.reset();
        self.rng = rng;
        self.game_over = false;
        self.started = true;
        self.current_piece = None;
        self.clearing_lines = None;
        self.events.clear();
        self.last_clear_type = None;
        self.last_was_rotation = false;
        self.last_kick = None;
        self.action_text_timer = Duration::ZERO;

        // Spawn first piece
        self.spawn_piece();
    }

    /// Spawn the next piece from the bag.
    fn spawn_piece(&mut self) {
        let piece_type = self.bag.next(&mut self.rng);
        let piece = Piece::new(piece_type);

        if self.board.is_blocked(&piece) {
            self.game_over = true;
            self.events.push(GameEvent::GameOver);
            return;
        }

        self.current_piece = Some(piece);
        self.lock_delay.reset();
        self.gravity.reset();
        self.last_was_rotation = false;
        self.last_kick = None;
    }

    /// Process a player action. Returns any events generated.
    pub fn handle_action(&mut self, action: GameAction) -> TickResult {
        if self.game_over || !self.started {
            return TickResult::none();
        }
        if self.clearing_lines.is_some() {
            return TickResult::none();
        }

        self.stats.inputs += 1;

        match action {
            GameAction::MoveLeft => self.do_move_left(),
            GameAction::MoveRight => self.do_move_right(),
            GameAction::SoftDrop => self.do_soft_drop(),
            GameAction::HardDrop => self.do_hard_drop(),
            GameAction::RotateCW => self.do_rotate_cw(),
            GameAction::RotateCCW => self.do_rotate_ccw(),
            GameAction::Rotate180 => self.do_rotate_180(),
            GameAction::Hold => self.do_hold(),
            GameAction::SoftDropRelease => {
                self.gravity.soft_dropping = false;
                TickResult::none()
            }
        }
    }

    /// Update game state for a frame. Call after processing all actions.
    pub fn update(&mut self, dt: Duration) -> TickResult {
        if self.game_over || !self.started {
            return TickResult::none();
        }

        self.stats.time += dt;

        // Action text timer
        if self.action_text_timer > Duration::ZERO {
            self.action_text_timer = self.action_text_timer.saturating_sub(dt);
            if self.action_text_timer == Duration::ZERO {
                self.last_clear_type = None;
            }
        }

        // Line clear animation
        if let Some((ref lines, ref mut timer)) = self.clearing_lines {
            if dt >= *timer {
                let lines = lines.clone();
                self.clearing_lines = None;
                self.board.clear_lines(&lines);

                // Deploy any ready garbage
                self.deploy_garbage();

                // Spawn next piece
                self.spawn_piece();
                return TickResult::none();
            } else {
                *timer -= dt;
                return TickResult::none();
            }
        }

        // Apply gravity
        if let Some(ref mut piece) = self.current_piece {
            let drops = self.gravity.tick(dt);
            for _ in 0..drops {
                if !movement::try_move_down(&self.board, piece) {
                    break;
                }
                self.last_was_rotation = false;
                if self.gravity.soft_dropping {
                    self.scoring.add_soft_drop(1);
                }
            }

            // Check grounding and lock delay
            let grounded = is_grounded(&self.board, piece);
            self.lock_delay.start_if_grounded(grounded);

            if grounded && self.lock_delay.tick(dt) {
                return self.lock_current_piece();
            }
        }

        // Tick garbage timers
        let ready_garbage = self.garbage.tick(dt);
        if ready_garbage > 0 && self.clearing_lines.is_none() {
            for _ in 0..ready_garbage {
                let gap = self.garbage.gap_column(&mut self.rng);
                self.board.add_garbage(1, gap);
            }
            self.stats.garbage_received += ready_garbage;
            self.events.push(GameEvent::GarbageReceived(ready_garbage));

            // Check if current piece is now overlapping
            if let Some(ref piece) = self.current_piece {
                if !self.board.piece_fits(piece) {
                    self.game_over = true;
                    self.events.push(GameEvent::GameOver);
                }
            }
        }

        TickResult::none()
    }

    fn do_move_left(&mut self) -> TickResult {
        if let Some(ref mut piece) = self.current_piece {
            if movement::try_move_left(&self.board, piece) {
                self.last_was_rotation = false;
                if is_grounded(&self.board, piece) {
                    self.lock_delay.try_reset();
                }
            }
        }
        TickResult::none()
    }

    fn do_move_right(&mut self) -> TickResult {
        if let Some(ref mut piece) = self.current_piece {
            if movement::try_move_right(&self.board, piece) {
                self.last_was_rotation = false;
                if is_grounded(&self.board, piece) {
                    self.lock_delay.try_reset();
                }
            }
        }
        TickResult::none()
    }

    fn do_soft_drop(&mut self) -> TickResult {
        self.gravity.soft_dropping = true;
        if let Some(ref mut piece) = self.current_piece {
            if movement::try_move_down(&self.board, piece) {
                self.last_was_rotation = false;
                self.scoring.add_soft_drop(1);
                self.gravity.reset();
            }
        }
        TickResult::none()
    }

    fn do_hard_drop(&mut self) -> TickResult {
        if let Some(ref mut piece) = self.current_piece {
            let cells = movement::hard_drop(&self.board, piece);
            self.scoring.add_hard_drop(cells);
            self.events.push(GameEvent::HardDrop { cells });
            let mut result = self.lock_current_piece();
            result.hard_dropped = true;
            return result;
        }
        TickResult::none()
    }

    fn do_rotate_cw(&mut self) -> TickResult {
        if let Some(ref mut piece) = self.current_piece {
            let target = piece.rotation.cw();
            if let Some(kick) = movement::try_rotate(&self.board, piece, target) {
                self.last_was_rotation = true;
                self.last_kick = Some(kick);
                if is_grounded(&self.board, piece) {
                    self.lock_delay.try_reset();
                }
            }
        }
        TickResult::none()
    }

    fn do_rotate_ccw(&mut self) -> TickResult {
        if let Some(ref mut piece) = self.current_piece {
            let target = piece.rotation.ccw();
            if let Some(kick) = movement::try_rotate(&self.board, piece, target) {
                self.last_was_rotation = true;
                self.last_kick = Some(kick);
                if is_grounded(&self.board, piece) {
                    self.lock_delay.try_reset();
                }
            }
        }
        TickResult::none()
    }

    fn do_rotate_180(&mut self) -> TickResult {
        if let Some(ref mut piece) = self.current_piece {
            let target = piece.rotation.flip();
            if let Some(kick) = movement::try_rotate(&self.board, piece, target) {
                self.last_was_rotation = true;
                self.last_kick = Some(kick);
                if is_grounded(&self.board, piece) {
                    self.lock_delay.try_reset();
                }
            }
        }
        TickResult::none()
    }

    fn do_hold(&mut self) -> TickResult {
        if let Some(ref piece) = self.current_piece {
            let piece_type = piece.piece_type;
            match self.hold.hold(piece_type) {
                Ok(prev) => {
                    match prev {
                        Some(held_type) => {
                            // Swap with held piece
                            let new_piece = Piece::new(held_type);
                            if !self.board.piece_fits(&new_piece) {
                                // Can't swap, undo hold
                                self.hold.piece = Some(piece_type);
                                self.hold.used_this_turn = false;
                                return TickResult::none();
                            }
                            self.current_piece = Some(new_piece);
                        }
                        None => {
                            // First hold, spawn next
                            self.current_piece = None;
                            self.spawn_piece();
                        }
                    }
                    self.lock_delay.reset();
                    self.gravity.reset();
                    self.last_was_rotation = false;
                    self.last_kick = None;
                }
                Err(()) => {} // Already held this turn
            }
        }
        TickResult::none()
    }

    /// Lock the current piece and process line clears.
    fn lock_current_piece(&mut self) -> TickResult {
        let piece = match self.current_piece.take() {
            Some(p) => p,
            None => return TickResult::none(),
        };

        // Detect spin before locking
        let spin = detect_spin(&self.board, &piece, self.last_was_rotation, self.last_kick);

        // Lock piece onto board
        self.board.lock_piece(&piece);
        self.stats.pieces_placed += 1;
        self.hold.reset_turn();
        self.events.push(GameEvent::PieceLocked);

        // Check for lockout (all cells above visible)
        let all_above = piece
            .cells()
            .iter()
            .all(|&(_, row)| row >= VISIBLE_HEIGHT as i32);
        if all_above {
            self.game_over = true;
            self.events.push(GameEvent::GameOver);
            return TickResult {
                game_over: true,
                piece_locked: true,
                ..TickResult::none()
            };
        }

        // Find full lines
        let full_lines = self.board.find_full_lines();
        let lines = full_lines.len() as u32;

        // Count garbage cleared
        let garbage_cleared = self.board.count_garbage_in_rows(&full_lines);
        self.stats.garbage_cleared += garbage_cleared as u32;

        // Classify the clear
        let clear_type = ClearType::classify(lines, spin, piece.piece_type);

        // Track clear type in stats
        match &clear_type {
            ClearType::Single => self.stats.singles += 1,
            ClearType::Double => self.stats.doubles += 1,
            ClearType::Triple => self.stats.triples += 1,
            ClearType::Quad => self.stats.quads += 1,
            ClearType::TSpin | ClearType::MiniTSpin => self.stats.tspins += 1,
            ClearType::TSpinSingle => {
                self.stats.tspin_singles += 1;
                self.stats.tspins += 1;
            }
            ClearType::TSpinDouble => {
                self.stats.tspin_doubles += 1;
                self.stats.tspins += 1;
            }
            ClearType::TSpinTriple => {
                self.stats.tspin_triples += 1;
                self.stats.tspins += 1;
            }
            ClearType::MiniTSpinSingle | ClearType::MiniTSpinDouble => {
                self.stats.mini_tspins += 1;
            }
            ClearType::AllSpin(_) => self.stats.all_spins += 1,
            ClearType::None => {}
        }

        // Check for perfect clear (board will be empty after clearing)
        let is_pc = if lines > 0 {
            // Simulate clearing to check
            let mut test_board = self.board.clone();
            test_board.clear_lines(&full_lines);
            test_board.is_empty()
        } else {
            false
        };

        if is_pc {
            self.stats.perfect_clears += 1;
            self.events.push(GameEvent::PerfectClear);
        }

        // Process scoring
        let (score_gained, attack) = self.scoring.process_clear(&clear_type, lines, is_pc);
        self.stats.score = self.scoring.score;
        self.stats.level = self.scoring.level;
        self.stats.lines_cleared = self.scoring.lines_cleared;

        // Track max combo/btb
        if self.scoring.combo >= 0 {
            self.stats.max_combo = self.stats.max_combo.max(self.scoring.combo as u32);
        }
        if self.scoring.btb >= 0 {
            self.stats.max_btb = self.stats.max_btb.max(self.scoring.btb as u32);
        }

        // Cancel garbage with attack, send remaining
        let remaining_attack = if attack > 0 {
            self.garbage.cancel(attack)
        } else {
            0
        };
        self.stats.attack_sent += remaining_attack;

        // Update gravity level
        self.gravity.level = self.scoring.level;

        // Emit events
        if spin != SpinType::None {
            self.events.push(GameEvent::TSpin(spin));
        }
        if self.scoring.combo > 0 {
            self.events
                .push(GameEvent::Combo(self.scoring.combo as u32));
        }
        if self.scoring.btb > 0 {
            self.events
                .push(GameEvent::BackToBack(self.scoring.btb as u32));
        }
        if lines > 0 {
            self.events.push(GameEvent::LinesClear(full_lines.clone()));
        }

        // Set action text
        if !matches!(clear_type, ClearType::None) {
            self.last_clear_type = Some(clear_type.clone());
            self.action_text_timer = Duration::from_millis(2000);
        }

        // Start line clear animation or spawn next piece
        if !full_lines.is_empty() {
            self.clearing_lines = Some((full_lines, self.clear_delay));
        } else {
            // Deploy pending garbage
            self.deploy_garbage();
            // Spawn next piece immediately
            self.spawn_piece();
        }

        TickResult {
            lines_cleared: lines,
            clear_type: ClearType::classify(lines, spin, piece.piece_type),
            is_perfect_clear: is_pc,
            score_gained,
            attack: remaining_attack,
            piece_locked: true,
            game_over: self.game_over,
            hard_dropped: false,
            spin_type: spin,
        }
    }

    /// Deploy any pending garbage that's ready.
    fn deploy_garbage(&mut self) {
        // Garbage is deployed via the tick method in update()
        // This is called after line clears to allow garbage insertion
    }

    /// Get preview pieces.
    pub fn preview(&self) -> Vec<PieceType> {
        self.bag.peek(3)
    }

    /// Get ghost piece Y position.
    pub fn ghost_y(&self) -> Option<i32> {
        self.current_piece.as_ref().map(|p| ghost_y(&self.board, p))
    }

    /// Check if the board is in a danger state (stack close to top).
    pub fn is_danger(&self) -> bool {
        self.board.max_height() >= VISIBLE_HEIGHT - 4
    }

    /// Drain all pending events.
    pub fn drain_events(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_start() {
        let mut game = GameState::new();
        game.start();
        assert!(game.started);
        assert!(!game.game_over);
        assert!(game.current_piece.is_some());
    }

    #[test]
    fn test_hard_drop_locks() {
        let mut game = GameState::new();
        game.start();
        let result = game.handle_action(GameAction::HardDrop);
        assert!(result.piece_locked);
        // Should have spawned a new piece
        assert!(game.current_piece.is_some() || game.game_over);
    }

    #[test]
    fn test_hold() {
        let mut game = GameState::new();
        game.start();
        let original_type = game.current_piece.as_ref().unwrap().piece_type;
        game.handle_action(GameAction::Hold);
        assert_eq!(game.hold.piece, Some(original_type));
        assert!(game.current_piece.is_some());
    }

    #[test]
    fn test_gravity_drops_piece() {
        let mut game = GameState::new();
        game.start();
        let start_y = game.current_piece.as_ref().unwrap().y;
        // Tick for 2 seconds (level 0 gravity = 1s per drop)
        game.update(Duration::from_millis(1100));
        let end_y = game.current_piece.as_ref().map(|p| p.y).unwrap_or(0);
        assert!(end_y < start_y || game.current_piece.is_none());
    }
}
