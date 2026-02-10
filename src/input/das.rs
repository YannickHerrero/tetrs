use std::time::Duration;

/// DAS (Delayed Auto-Shift) configuration.
pub const DAS_DELAY: Duration = Duration::from_millis(133);
/// ARR (Auto Repeat Rate). 0 = instant.
pub const ARR_DELAY: Duration = Duration::from_millis(0);
/// Soft drop ARR.
pub const SD_ARR_DELAY: Duration = Duration::from_millis(0);

/// DAS state for a single direction.
#[derive(Debug, Clone)]
pub struct DasState {
    pub pressed: bool,
    pub phase: DasPhase,
    pub timer: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DasPhase {
    Idle,
    /// Waiting for DAS delay to expire.
    Charging,
    /// Auto-repeating.
    Repeating,
}

impl DasState {
    pub fn new() -> Self {
        Self {
            pressed: false,
            phase: DasPhase::Idle,
            timer: Duration::ZERO,
        }
    }

    /// Key pressed.
    pub fn press(&mut self) {
        self.pressed = true;
        self.phase = DasPhase::Charging;
        self.timer = Duration::ZERO;
    }

    /// Key released.
    pub fn release(&mut self) {
        self.pressed = false;
        self.phase = DasPhase::Idle;
        self.timer = Duration::ZERO;
    }

    /// Tick the DAS timer. Returns the number of moves to execute.
    pub fn tick(&mut self, dt: Duration, das_delay: Duration, arr_delay: Duration) -> u32 {
        if !self.pressed {
            return 0;
        }

        match self.phase {
            DasPhase::Idle => 0,
            DasPhase::Charging => {
                self.timer += dt;
                if self.timer >= das_delay {
                    self.phase = DasPhase::Repeating;
                    let overshoot = self.timer - das_delay;
                    self.timer = overshoot;
                    if arr_delay.is_zero() {
                        // Instant ARR: return a large number (will be capped by wall)
                        20
                    } else {
                        // Count how many ARR ticks fit in the overshoot
                        1 + (overshoot.as_nanos() / arr_delay.as_nanos()) as u32
                    }
                } else {
                    0
                }
            }
            DasPhase::Repeating => {
                if arr_delay.is_zero() {
                    // Instant: always move to wall
                    20
                } else {
                    self.timer += dt;
                    let mut moves = 0;
                    while self.timer >= arr_delay {
                        self.timer -= arr_delay;
                        moves += 1;
                    }
                    moves
                }
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.pressed
    }

    pub fn reset(&mut self) {
        self.pressed = false;
        self.phase = DasPhase::Idle;
        self.timer = Duration::ZERO;
    }
}

/// DAS handler for both horizontal directions and soft drop.
#[derive(Debug, Clone)]
pub struct DasHandler {
    pub left: DasState,
    pub right: DasState,
    pub soft_drop: DasState,
    pub das_delay: Duration,
    pub arr_delay: Duration,
    pub sd_arr_delay: Duration,
}

impl DasHandler {
    pub fn new() -> Self {
        Self {
            left: DasState::new(),
            right: DasState::new(),
            soft_drop: DasState::new(),
            das_delay: DAS_DELAY,
            arr_delay: ARR_DELAY,
            sd_arr_delay: SD_ARR_DELAY,
        }
    }

    /// Tick all DAS states. Returns (left_moves, right_moves, soft_drop_moves).
    pub fn tick(&mut self, dt: Duration) -> (u32, u32, u32) {
        let left = self.left.tick(dt, self.das_delay, self.arr_delay);
        let right = self.right.tick(dt, self.das_delay, self.arr_delay);
        let sd = self.soft_drop.tick(dt, self.das_delay, self.sd_arr_delay);
        (left, right, sd)
    }

    /// Reset all DAS state.
    pub fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
        self.soft_drop.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_das_charging() {
        let mut das = DasState::new();
        das.press();
        // Not enough time
        let moves = das.tick(
            Duration::from_millis(100),
            DAS_DELAY,
            Duration::from_millis(50),
        );
        assert_eq!(moves, 0);
        // Enough time to trigger
        let moves = das.tick(
            Duration::from_millis(50),
            DAS_DELAY,
            Duration::from_millis(50),
        );
        assert!(moves >= 1);
    }

    #[test]
    fn test_das_instant_arr() {
        let mut das = DasState::new();
        das.press();
        // Charge past DAS
        let moves = das.tick(Duration::from_millis(200), DAS_DELAY, Duration::ZERO);
        assert_eq!(moves, 20); // Instant = 20 (capped)
    }

    #[test]
    fn test_das_release() {
        let mut das = DasState::new();
        das.press();
        das.release();
        let moves = das.tick(
            Duration::from_millis(200),
            DAS_DELAY,
            Duration::from_millis(50),
        );
        assert_eq!(moves, 0);
    }
}
