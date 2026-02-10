use crate::ai::difficulty::EvalWeights;
use crate::game::board::{Board, BOARD_HEIGHT, BOARD_WIDTH};

/// Evaluate a board state and return a score (higher = better).
pub fn evaluate(board: &Board, lines_cleared: u32, weights: &EvalWeights) -> f64 {
    let aggregate_height = board.aggregate_height() as f64;
    let holes = board.count_holes() as f64;
    let bumpiness = board.bumpiness() as f64;
    let wells = count_wells(board) as f64;
    let col_transitions = column_transitions(board) as f64;
    let row_transitions = row_transitions(board) as f64;
    let is_perfect_clear = board.is_empty() && lines_cleared > 0;

    let mut score = 0.0;
    score += weights.aggregate_height * aggregate_height;
    score += weights.holes * holes;
    score += weights.bumpiness * bumpiness;
    score += weights.lines_cleared * lines_cleared as f64;
    score += weights.wells * wells;
    score += weights.column_transitions * col_transitions;
    score += weights.row_transitions * row_transitions;

    if is_perfect_clear {
        score += weights.perfect_clear;
    }

    score
}

/// Count wells: sum of well depths. A well is a column lower than both neighbors.
fn count_wells(board: &Board) -> usize {
    let heights: Vec<usize> = (0..BOARD_WIDTH).map(|c| board.column_height(c)).collect();
    let mut wells = 0;

    for col in 0..BOARD_WIDTH {
        let left_h = if col > 0 {
            heights[col - 1]
        } else {
            BOARD_HEIGHT
        };
        let right_h = if col < BOARD_WIDTH - 1 {
            heights[col + 1]
        } else {
            BOARD_HEIGHT
        };
        let h = heights[col];

        if h < left_h && h < right_h {
            wells += left_h.min(right_h) - h;
        }
    }

    wells
}

/// Count column transitions: number of filled/empty transitions vertically.
fn column_transitions(board: &Board) -> usize {
    let mut transitions = 0;
    for col in 0..BOARD_WIDTH {
        let mut prev_filled = true; // Floor is filled
        for row in 0..BOARD_HEIGHT {
            let filled = board.grid[row][col].is_occupied();
            if filled != prev_filled {
                transitions += 1;
            }
            prev_filled = filled;
        }
    }
    transitions
}

/// Count row transitions: number of filled/empty transitions horizontally.
fn row_transitions(board: &Board) -> usize {
    let mut transitions = 0;
    for row in 0..BOARD_HEIGHT {
        let mut prev_filled = true; // Walls are filled
        for col in 0..BOARD_WIDTH {
            let filled = board.grid[row][col].is_occupied();
            if filled != prev_filled {
                transitions += 1;
            }
            prev_filled = filled;
        }
        // Right wall
        if !prev_filled {
            transitions += 1;
        }
    }
    transitions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::difficulty::AiDifficulty;

    #[test]
    fn test_empty_board_evaluation() {
        let board = Board::new();
        let weights = AiDifficulty::Hard.weights();
        let score = evaluate(&board, 0, &weights);
        // Empty board should have a relatively neutral score
        assert!(score.is_finite());
    }

    #[test]
    fn test_line_clear_improves_score() {
        let board = Board::new();
        let weights = AiDifficulty::Hard.weights();
        let score_no_clear = evaluate(&board, 0, &weights);
        let score_with_clear = evaluate(&board, 4, &weights);
        assert!(score_with_clear > score_no_clear);
    }
}
