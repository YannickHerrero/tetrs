use std::time::Duration;

/// AI difficulty presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AiDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl AiDifficulty {
    pub fn name(self) -> &'static str {
        match self {
            AiDifficulty::Easy => "Easy",
            AiDifficulty::Medium => "Medium",
            AiDifficulty::Hard => "Hard",
            AiDifficulty::Expert => "Expert",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            AiDifficulty::Easy => "Slow, makes mistakes, sends little garbage",
            AiDifficulty::Medium => "Moderate speed, decent play",
            AiDifficulty::Hard => "Fast, efficient, aggressive garbage",
            AiDifficulty::Expert => "Relentless, near-optimal play",
        }
    }

    /// Time the AI "thinks" before starting to move.
    pub fn think_time(self) -> Duration {
        match self {
            AiDifficulty::Easy => Duration::from_millis(800),
            AiDifficulty::Medium => Duration::from_millis(400),
            AiDifficulty::Hard => Duration::from_millis(150),
            AiDifficulty::Expert => Duration::from_millis(50),
        }
    }

    /// How fast the AI moves pieces (cells per second).
    pub fn move_speed(self) -> f64 {
        match self {
            AiDifficulty::Easy => 3.0,
            AiDifficulty::Medium => 6.0,
            AiDifficulty::Hard => 15.0,
            AiDifficulty::Expert => 30.0,
        }
    }

    /// Probability of making a mistake (choosing suboptimal placement).
    pub fn error_rate(self) -> f64 {
        match self {
            AiDifficulty::Easy => 0.15,
            AiDifficulty::Medium => 0.05,
            AiDifficulty::Hard => 0.01,
            AiDifficulty::Expert => 0.0,
        }
    }

    /// Whether the AI uses hold.
    pub fn uses_hold(self) -> bool {
        match self {
            AiDifficulty::Easy => false,
            _ => true,
        }
    }

    /// Evaluation weights for this difficulty.
    pub fn weights(self) -> EvalWeights {
        match self {
            AiDifficulty::Easy => EvalWeights {
                aggregate_height: -0.30,
                holes: -0.25,
                bumpiness: -0.10,
                lines_cleared: 0.50,
                wells: 0.05,
                column_transitions: -0.05,
                row_transitions: -0.03,
                perfect_clear: 2.0,
            },
            AiDifficulty::Medium => EvalWeights {
                aggregate_height: -0.45,
                holes: -0.35,
                bumpiness: -0.15,
                lines_cleared: 0.65,
                wells: 0.08,
                column_transitions: -0.08,
                row_transitions: -0.04,
                perfect_clear: 4.0,
            },
            AiDifficulty::Hard => EvalWeights {
                aggregate_height: -0.51,
                holes: -0.36,
                bumpiness: -0.18,
                lines_cleared: 0.76,
                wells: 0.10,
                column_transitions: -0.10,
                row_transitions: -0.05,
                perfect_clear: 5.0,
            },
            AiDifficulty::Expert => EvalWeights {
                aggregate_height: -0.51,
                holes: -0.36,
                bumpiness: -0.18,
                lines_cleared: 0.76,
                wells: 0.12,
                column_transitions: -0.12,
                row_transitions: -0.06,
                perfect_clear: 6.0,
            },
        }
    }
}

/// Heuristic evaluation weights.
#[derive(Debug, Clone, Copy)]
pub struct EvalWeights {
    pub aggregate_height: f64,
    pub holes: f64,
    pub bumpiness: f64,
    pub lines_cleared: f64,
    pub wells: f64,
    pub column_transitions: f64,
    pub row_transitions: f64,
    pub perfect_clear: f64,
}
