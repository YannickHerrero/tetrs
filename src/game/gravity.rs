use std::time::Duration;

/// Gravity system: controls automatic piece falling.
#[derive(Debug, Clone)]
pub struct Gravity {
    /// Time accumulated since last gravity drop.
    pub accumulator: Duration,
    /// Current level (affects gravity speed).
    pub level: u32,
    /// Whether soft drop is active.
    pub soft_dropping: bool,
}

/// Soft drop speed multiplier.
const SOFT_DROP_FACTOR: f64 = 20.0;

impl Gravity {
    pub fn new() -> Self {
        Self {
            accumulator: Duration::ZERO,
            level: 0,
            soft_dropping: false,
        }
    }

    /// Get the gravity interval for the current level.
    /// Uses the guideline formula: (0.8 - (level * 0.007))^level seconds.
    pub fn interval(&self) -> Duration {
        let level = self.level as f64;
        let seconds = (0.8 - (level * 0.007)).max(0.001).powf(level);
        let seconds = if self.soft_dropping {
            seconds / SOFT_DROP_FACTOR
        } else {
            seconds
        };
        Duration::from_secs_f64(seconds.max(0.001))
    }

    /// Tick the gravity timer. Returns the number of cells to drop.
    pub fn tick(&mut self, dt: Duration) -> u32 {
        self.accumulator += dt;
        let interval = self.interval();
        let mut drops = 0;
        while self.accumulator >= interval {
            self.accumulator -= interval;
            drops += 1;
        }
        // Cap drops to avoid huge jumps
        drops.min(20)
    }

    /// Reset accumulator (e.g., after manual move down).
    pub fn reset(&mut self) {
        self.accumulator = Duration::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_level_0() {
        let g = Gravity::new();
        let interval = g.interval();
        // Level 0: (0.8)^0 = 1.0 second
        assert!((interval.as_secs_f64() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_gravity_increases_with_level() {
        let mut g1 = Gravity::new();
        g1.level = 0;
        let mut g2 = Gravity::new();
        g2.level = 5;
        assert!(g2.interval() < g1.interval());
    }

    #[test]
    fn test_soft_drop_faster() {
        let mut g = Gravity::new();
        let normal = g.interval();
        g.soft_dropping = true;
        let soft = g.interval();
        assert!(soft < normal);
    }

    #[test]
    fn test_gravity_tick() {
        let mut g = Gravity::new();
        // At level 0, interval is 1 second
        let drops = g.tick(Duration::from_millis(500));
        assert_eq!(drops, 0);
        let drops = g.tick(Duration::from_millis(600));
        assert_eq!(drops, 1);
    }
}
