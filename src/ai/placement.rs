use crate::ai::difficulty::EvalWeights;
use crate::ai::evaluator;
use crate::game::board::Board;
use crate::game::movement;
use crate::game::piece::{Piece, PieceType, RotationState};

/// A potential placement for a piece.
#[derive(Debug, Clone)]
pub struct Placement {
    pub piece_type: PieceType,
    pub rotation: RotationState,
    pub x: i32,
    pub y: i32,
    pub score: f64,
    pub use_hold: bool,
}

/// Generate all legal placements for a piece on a board.
pub fn generate_placements(
    board: &Board,
    piece_type: PieceType,
    weights: &EvalWeights,
    use_hold: bool,
) -> Vec<Placement> {
    let rotations = [
        RotationState::R0,
        RotationState::R1,
        RotationState::R2,
        RotationState::R3,
    ];

    let mut placements = Vec::new();

    for &rotation in &rotations {
        // Try all horizontal positions
        for x in -2..12 {
            let mut piece = Piece::new(piece_type);
            piece.x = x;
            piece.rotation = rotation;

            // Check if the piece fits at spawn height
            if !board.fits_at(&piece, x, piece.y, rotation) {
                // Try lower spawn positions
                let mut found = false;
                for spawn_y in (0..piece.y).rev() {
                    if board.fits_at(&piece, x, spawn_y, rotation) {
                        piece.y = spawn_y;
                        found = true;
                        break;
                    }
                }
                if !found {
                    continue;
                }
            }

            // Hard drop to find landing position
            let mut test_piece = piece.clone();
            movement::hard_drop(board, &mut test_piece);

            // Check if this is a valid final position
            if !board.fits_at(&test_piece, test_piece.x, test_piece.y, test_piece.rotation) {
                continue;
            }

            // Simulate locking and evaluate
            let mut test_board = board.clone();
            test_board.lock_piece(&test_piece);

            let full_lines = test_board.find_full_lines();
            let lines = full_lines.len() as u32;
            test_board.clear_lines(&full_lines);

            let score = evaluator::evaluate(&test_board, lines, weights);

            placements.push(Placement {
                piece_type,
                rotation,
                x: test_piece.x,
                y: test_piece.y,
                score,
                use_hold: use_hold,
            });
        }
    }

    // Deduplicate by (x, y, rotation) keeping best score
    placements.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    placements.dedup_by(|a, b| a.x == b.x && a.y == b.y && a.rotation == b.rotation);

    placements
}

/// Find the best placement for a piece, optionally considering hold.
pub fn find_best_placement(
    board: &Board,
    current_type: PieceType,
    hold_type: Option<PieceType>,
    weights: &EvalWeights,
    use_hold: bool,
) -> Option<Placement> {
    let mut best: Option<Placement> = None;

    // Try current piece
    let placements = generate_placements(board, current_type, weights, false);
    if let Some(p) = placements.first() {
        best = Some(p.clone());
    }

    // Try hold piece if available and allowed
    if use_hold {
        if let Some(hold_type) = hold_type {
            let hold_placements = generate_placements(board, hold_type, weights, true);
            if let Some(p) = hold_placements.first() {
                if best.as_ref().map_or(true, |b| p.score > b.score) {
                    best = Some(p.clone());
                }
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::difficulty::AiDifficulty;

    #[test]
    fn test_generate_placements() {
        let board = Board::new();
        let weights = AiDifficulty::Hard.weights();
        let placements = generate_placements(&board, PieceType::T, &weights, false);
        assert!(!placements.is_empty());
    }

    #[test]
    fn test_find_best() {
        let board = Board::new();
        let weights = AiDifficulty::Hard.weights();
        let best = find_best_placement(&board, PieceType::T, None, &weights, false);
        assert!(best.is_some());
    }
}
