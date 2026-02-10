use super::clear::ClearType;

/// Attack damage table indexed by [clear_type][combo_level].
/// Based on modern guideline (TETR.IO-style).
const ATTACK_TABLE: [[u32; 21]; 8] = [
    // Single
    [
        0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
    ],
    // Double
    [
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6,
    ],
    // Triple
    [
        2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12,
    ],
    // Quad (Tetris)
    [
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ],
    // T-Spin Single
    [
        2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12,
    ],
    // T-Spin Double
    [
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ],
    // T-Spin Triple
    [
        6, 7, 9, 10, 12, 13, 15, 16, 18, 19, 21, 22, 24, 25, 27, 28, 30, 31, 33, 34, 36,
    ],
    // T-Spin Mini Single
    [
        0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
    ],
];

/// Base score values per clear type.
fn base_score(clear_type: &ClearType) -> u32 {
    match clear_type {
        ClearType::Single => 100,
        ClearType::Double => 300,
        ClearType::Triple => 500,
        ClearType::Quad => 800,
        ClearType::TSpin => 400,
        ClearType::TSpinSingle => 800,
        ClearType::TSpinDouble => 1200,
        ClearType::TSpinTriple => 1600,
        ClearType::MiniTSpin => 100,
        ClearType::MiniTSpinSingle => 200,
        ClearType::MiniTSpinDouble => 400,
        ClearType::AllSpin(_) => 400,
        ClearType::None => 0,
    }
}

/// Map clear type to attack table row index.
fn attack_index(clear_type: &ClearType) -> Option<usize> {
    match clear_type {
        ClearType::Single => Some(0),
        ClearType::Double => Some(1),
        ClearType::Triple => Some(2),
        ClearType::Quad => Some(3),
        ClearType::TSpinSingle => Some(4),
        ClearType::TSpinDouble => Some(5),
        ClearType::TSpinTriple => Some(6),
        ClearType::MiniTSpinSingle => Some(7),
        _ => None,
    }
}

/// Scoring and combo/BTB state.
#[derive(Debug, Clone)]
pub struct Scoring {
    pub score: u64,
    pub combo: i32, // -1 = no combo
    pub btb: i32,   // -1 = no BTB, 0+ = BTB chain length
    pub level: u32,
    pub lines_cleared: u32,
    pub lines_per_level: u32,
}

impl Scoring {
    pub fn new() -> Self {
        Self {
            score: 0,
            combo: -1,
            btb: -1,
            level: 0,
            lines_cleared: 0,
            lines_per_level: 10,
        }
    }

    /// Process a line clear. Returns (score_gained, attack_damage).
    pub fn process_clear(
        &mut self,
        clear_type: &ClearType,
        lines: u32,
        is_perfect_clear: bool,
    ) -> (u64, u32) {
        if matches!(clear_type, ClearType::None) && lines == 0 {
            // No clear: reset combo
            if self.combo >= 0 {
                self.combo = -1;
            }
            return (0, 0);
        }

        // Increment combo
        self.combo += 1;

        // Calculate BTB
        let is_difficult = clear_type.is_difficult();
        if lines > 0 {
            if is_difficult {
                self.btb += 1;
            } else {
                self.btb = -1;
            }
        }

        // Base score
        let mut score = base_score(clear_type) as u64;

        // Perfect clear bonus
        if is_perfect_clear {
            score += 3500;
        }

        // Combo bonus
        if self.combo > 0 {
            score += 50 * self.combo as u64;
        }

        // BTB multiplier
        if self.btb > 0 {
            score = score * 3 / 2;
        }

        // Level multiplier
        score *= (self.level + 1) as u64;

        self.score += score;

        // Calculate attack
        let mut attack = self.calculate_attack(clear_type, is_perfect_clear);

        // Track lines and level
        self.lines_cleared += lines;
        self.check_level_up();

        // Ensure we return at least something for non-zero clears in attack
        if lines > 0 && attack == 0 && self.combo > 2 {
            attack = 1; // Minimum attack for high combos
        }

        (score, attack)
    }

    /// Calculate attack damage for versus mode.
    fn calculate_attack(&self, clear_type: &ClearType, is_perfect_clear: bool) -> u32 {
        let combo_idx = (self.combo.max(0) as usize).min(20);

        let base_attack = match attack_index(clear_type) {
            Some(idx) => ATTACK_TABLE[idx][combo_idx],
            None => 0,
        };

        // BTB bonus (logarithmic scaling from TETR.IO)
        let btb_bonus = if self.btb > 0 {
            let x = (1.0 + self.btb as f64 * 0.8).ln();
            let bonus = (x + 1.0).floor() + (1.0 + (x % 1.0)) / 3.0;
            bonus.floor() as u32
        } else {
            0
        };

        // Perfect clear
        let pc_bonus = if is_perfect_clear { 10 } else { 0 };

        base_attack + btb_bonus + pc_bonus
    }

    /// Add points for hard drop.
    pub fn add_hard_drop(&mut self, cells: u32) {
        self.score += cells as u64 * 2;
    }

    /// Add points for soft drop.
    pub fn add_soft_drop(&mut self, cells: u32) {
        self.score += cells as u64;
    }

    /// Check if we should level up.
    fn check_level_up(&mut self) {
        let target = (self.level + 1) * self.lines_per_level;
        if self.lines_cleared >= target {
            self.level += 1;
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_score() {
        let mut scoring = Scoring::new();
        let (score, _) = scoring.process_clear(&ClearType::Single, 1, false);
        assert_eq!(score, 100); // 100 * (0+1)
    }

    #[test]
    fn test_tetris_score() {
        let mut scoring = Scoring::new();
        let (score, _) = scoring.process_clear(&ClearType::Quad, 4, false);
        assert_eq!(score, 800);
    }

    #[test]
    fn test_combo_bonus() {
        let mut scoring = Scoring::new();
        scoring.process_clear(&ClearType::Single, 1, false);
        let (score, _) = scoring.process_clear(&ClearType::Single, 1, false);
        // Second clear: base 100 + combo 50*1 = 150
        assert_eq!(score, 150);
    }

    #[test]
    fn test_btb() {
        let mut scoring = Scoring::new();
        scoring.process_clear(&ClearType::Quad, 4, false);
        let (score, _) = scoring.process_clear(&ClearType::Quad, 4, false);
        // BTB: 800 * 1.5 = 1200 (plus combo)
        assert!(score > 800);
    }

    #[test]
    fn test_perfect_clear() {
        let mut scoring = Scoring::new();
        let (score, attack) = scoring.process_clear(&ClearType::Quad, 4, true);
        assert!(score >= 3500 + 800); // At least PC bonus + quad
        assert!(attack >= 10); // PC attack bonus
    }

    #[test]
    fn test_combo_reset() {
        let mut scoring = Scoring::new();
        scoring.process_clear(&ClearType::Single, 1, false);
        assert_eq!(scoring.combo, 0);
        scoring.process_clear(&ClearType::None, 0, false);
        assert_eq!(scoring.combo, -1);
    }

    #[test]
    fn test_level_up() {
        let mut scoring = Scoring::new();
        for _ in 0..10 {
            scoring.process_clear(&ClearType::Single, 1, false);
        }
        assert_eq!(scoring.level, 1);
    }
}
