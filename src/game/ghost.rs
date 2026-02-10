use super::board::Board;
use super::piece::Piece;

/// Calculate the ghost piece Y position (the lowest row where the piece still fits).
pub fn ghost_y(board: &Board, piece: &Piece) -> i32 {
    let mut y = piece.y;
    while board.fits_at(piece, piece.x, y - 1, piece.rotation) {
        y -= 1;
    }
    y
}

/// Get ghost piece cells at the drop target position.
pub fn ghost_cells(board: &Board, piece: &Piece) -> [(i32, i32); 4] {
    let gy = ghost_y(board, piece);
    piece.cells_at(piece.x, gy, piece.rotation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::piece::{Piece, PieceType};

    #[test]
    fn test_ghost_on_empty_board() {
        let board = Board::new();
        let piece = Piece::new(PieceType::T);
        let gy = ghost_y(&board, &piece);
        // T piece at x=3, should drop to row 0
        assert_eq!(gy, 0);
    }

    #[test]
    fn test_ghost_with_obstacle() {
        let mut board = Board::new();
        // Fill row 0
        for col in 0..10 {
            board.set(col, 0, crate::game::board::Cell::Filled(PieceType::I));
        }
        let piece = Piece::new(PieceType::T);
        let gy = ghost_y(&board, &piece);
        // T piece should land on row 1
        assert_eq!(gy, 1);
    }
}
