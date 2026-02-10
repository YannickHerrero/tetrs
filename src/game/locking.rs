use std::time::Duration;

/// Lock delay configuration.
const LOCK_DELAY: Duration = Duration::from_millis(500);
const MAX_LOCK_RESETS: u32 = 15;

/// Lock delay system: handles the delay before a grounded piece locks.
#[derive(Debug, Clone)]
pub struct LockDelay {
    /// Time remaining before lock.
    pub timer: Duration,
    /// Number of resets performed.
    pub resets: u32,
    /// Whether the piece is currently on the ground.
    pub grounded: bool,
    /// Whether the lock delay is active.
    pub active: bool,
}

impl LockDelay {
    pub fn new() -> Self {
        Self {
            timer: LOCK_DELAY,
            resets: 0,
            grounded: false,
            active: false,
        }
    }

    /// Called when the piece touches the ground.
    pub fn start_if_grounded(&mut self, is_grounded: bool) {
        if is_grounded && !self.active {
            self.active = true;
            self.grounded = true;
        } else if !is_grounded {
            // Piece lifted off ground (e.g., rotation kick), reset timer
            self.active = false;
            self.grounded = false;
            self.timer = LOCK_DELAY;
        }
        self.grounded = is_grounded;
    }

    /// Called when the piece is moved or rotated while on the ground.
    /// Returns true if the reset was accepted.
    pub fn try_reset(&mut self) -> bool {
        if self.active && self.resets < MAX_LOCK_RESETS {
            self.timer = LOCK_DELAY;
            self.resets += 1;
            true
        } else {
            false
        }
    }

    /// Tick the lock timer. Returns true if the piece should lock.
    pub fn tick(&mut self, dt: Duration) -> bool {
        if !self.active {
            return false;
        }
        if dt >= self.timer {
            self.timer = Duration::ZERO;
            true
        } else {
            self.timer -= dt;
            false
        }
    }

    /// Get lock progress as 0.0 (just started) to 1.0 (about to lock).
    pub fn progress(&self) -> f64 {
        if !self.active {
            return 0.0;
        }
        1.0 - (self.timer.as_secs_f64() / LOCK_DELAY.as_secs_f64())
    }

    /// Full reset for new piece.
    pub fn reset(&mut self) {
        self.timer = LOCK_DELAY;
        self.resets = 0;
        self.grounded = false;
        self.active = false;
    }

    pub fn resets_remaining(&self) -> u32 {
        MAX_LOCK_RESETS.saturating_sub(self.resets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_delay_activation() {
        let mut ld = LockDelay::new();
        assert!(!ld.active);
        ld.start_if_grounded(true);
        assert!(ld.active);
    }

    #[test]
    fn test_lock_delay_tick() {
        let mut ld = LockDelay::new();
        ld.start_if_grounded(true);
        // Tick less than lock delay
        assert!(!ld.tick(Duration::from_millis(200)));
        // Tick past remaining
        assert!(ld.tick(Duration::from_millis(400)));
    }

    #[test]
    fn test_lock_reset() {
        let mut ld = LockDelay::new();
        ld.start_if_grounded(true);
        ld.tick(Duration::from_millis(400));
        assert!(ld.try_reset());
        // Timer should be reset, so this shouldn't lock
        assert!(!ld.tick(Duration::from_millis(400)));
    }

    #[test]
    fn test_max_resets() {
        let mut ld = LockDelay::new();
        ld.start_if_grounded(true);
        for _ in 0..MAX_LOCK_RESETS {
            assert!(ld.try_reset());
        }
        // Should no longer accept resets
        assert!(!ld.try_reset());
    }
}
