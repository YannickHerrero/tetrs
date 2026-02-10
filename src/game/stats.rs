use std::time::Duration;

/// Game statistics tracking.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Stats {
    pub pieces_placed: u32,
    pub lines_cleared: u32,
    pub score: u64,
    pub level: u32,
    pub time: Duration,
    pub attack_sent: u32,
    pub garbage_received: u32,
    pub garbage_cleared: u32,

    // Clear type counts
    pub singles: u32,
    pub doubles: u32,
    pub triples: u32,
    pub quads: u32,
    pub tspins: u32,
    pub tspin_singles: u32,
    pub tspin_doubles: u32,
    pub tspin_triples: u32,
    pub mini_tspins: u32,
    pub all_spins: u32,
    pub perfect_clears: u32,

    // Combo/BTB tracking
    pub max_combo: u32,
    pub max_btb: u32,

    // Input tracking
    pub inputs: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            pieces_placed: 0,
            lines_cleared: 0,
            score: 0,
            level: 0,
            time: Duration::ZERO,
            attack_sent: 0,
            garbage_received: 0,
            garbage_cleared: 0,
            singles: 0,
            doubles: 0,
            triples: 0,
            quads: 0,
            tspins: 0,
            tspin_singles: 0,
            tspin_doubles: 0,
            tspin_triples: 0,
            mini_tspins: 0,
            all_spins: 0,
            perfect_clears: 0,
            max_combo: 0,
            max_btb: 0,
            inputs: 0,
        }
    }

    /// Pieces per second.
    pub fn pps(&self) -> f64 {
        let secs = self.time.as_secs_f64();
        if secs > 0.0 {
            self.pieces_placed as f64 / secs
        } else {
            0.0
        }
    }

    /// Attack per minute.
    pub fn apm(&self) -> f64 {
        let mins = self.time.as_secs_f64() / 60.0;
        if mins > 0.0 {
            self.attack_sent as f64 / mins
        } else {
            0.0
        }
    }

    /// Lines per minute.
    pub fn lpm(&self) -> f64 {
        let mins = self.time.as_secs_f64() / 60.0;
        if mins > 0.0 {
            self.lines_cleared as f64 / mins
        } else {
            0.0
        }
    }

    /// Keys per piece.
    pub fn kpp(&self) -> f64 {
        if self.pieces_placed > 0 {
            self.inputs as f64 / self.pieces_placed as f64
        } else {
            0.0
        }
    }

    /// Format time as MM:SS.mmm
    pub fn format_time(&self) -> String {
        let total_ms = self.time.as_millis();
        let minutes = total_ms / 60_000;
        let seconds = (total_ms % 60_000) / 1000;
        let millis = total_ms % 1000;
        format!("{:02}:{:02}.{:03}", minutes, seconds, millis)
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
