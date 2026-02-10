use super::piece::{Piece, PieceType, RotationState};
use super::srs;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 40; // 20 visible + 20 buffer
pub const VISIBLE_HEIGHT: usize = 20;

/// A single cell on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Filled(PieceType),
    Garbage,
}

impl Cell {
    pub fn is_empty(self) -> bool {
        matches!(self, Cell::Empty)
    }

    pub fn is_occupied(self) -> bool {
        !self.is_empty()
    }
}

/// The game board / playfield.
#[derive(Debug, Clone)]
pub struct Board {
    /// Grid stored row-major, row 0 = bottom.
    pub grid: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    pub fn get(&self, col: i32, row: i32) -> Cell {
        if col < 0 || col >= BOARD_WIDTH as i32 || row < 0 || row >= BOARD_HEIGHT as i32 {
            return Cell::Filled(PieceType::I); // Out of bounds = solid
        }
        self.grid[row as usize][col as usize]
    }

    pub fn set(&mut self, col: i32, row: i32, cell: Cell) {
        if col >= 0 && col < BOARD_WIDTH as i32 && row >= 0 && row < BOARD_HEIGHT as i32 {
            self.grid[row as usize][col as usize] = cell;
        }
    }

    /// Check if a piece fits at its current position.
    pub fn piece_fits(&self, piece: &Piece) -> bool {
        for (col, row) in piece.cells() {
            if col < 0 || col >= BOARD_WIDTH as i32 || row < 0 || row >= BOARD_HEIGHT as i32 {
                return false;
            }
            if self.grid[row as usize][col as usize].is_occupied() {
                return false;
            }
        }
        true
    }

    /// Check if a piece fits at the given position and rotation.
    pub fn fits_at(&self, piece: &Piece, x: i32, y: i32, rotation: RotationState) -> bool {
        for (col, row) in piece.cells_at(x, y, rotation) {
            if col < 0 || col >= BOARD_WIDTH as i32 || row < 0 || row >= BOARD_HEIGHT as i32 {
                return false;
            }
            if self.grid[row as usize][col as usize].is_occupied() {
                return false;
            }
        }
        true
    }

    /// Try rotating a piece with SRS kicks. Returns the successful kick offset if any.
    pub fn try_rotate(&self, piece: &Piece, target_rotation: RotationState) -> Option<(i32, i32)> {
        let kicks = srs::get_kicks(piece.piece_type, piece.rotation, target_rotation);
        for &(dx, dy) in kicks {
            if self.fits_at(piece, piece.x + dx, piece.y + dy, target_rotation) {
                return Some((dx, dy));
            }
        }
        None
    }

    /// Lock a piece onto the board.
    pub fn lock_piece(&mut self, piece: &Piece) {
        for (col, row) in piece.cells() {
            self.set(col, row, Cell::Filled(piece.piece_type));
        }
    }

    /// Check for and clear full lines. Returns the row indices that were cleared (sorted ascending).
    pub fn find_full_lines(&self) -> Vec<usize> {
        let mut full = Vec::new();
        for row in 0..BOARD_HEIGHT {
            if self.grid[row].iter().all(|c| c.is_occupied()) {
                full.push(row);
            }
        }
        full
    }

    /// Remove the given rows and collapse everything above down.
    pub fn clear_lines(&mut self, rows: &[usize]) {
        if rows.is_empty() {
            return;
        }
        // Build new grid excluding cleared rows
        let mut new_grid = [[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT];
        let mut dest = 0;
        for src in 0..BOARD_HEIGHT {
            if !rows.contains(&src) {
                new_grid[dest] = self.grid[src];
                dest += 1;
            }
        }
        self.grid = new_grid;
    }

    /// Check if the board is completely empty (for Perfect Clear detection).
    pub fn is_empty(&self) -> bool {
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                if self.grid[row][col].is_occupied() {
                    return false;
                }
            }
        }
        true
    }

    /// Get the height of a column (highest occupied row + 1, or 0 if empty).
    pub fn column_height(&self, col: usize) -> usize {
        for row in (0..BOARD_HEIGHT).rev() {
            if self.grid[row][col].is_occupied() {
                return row + 1;
            }
        }
        0
    }

    /// Get the max height across all columns.
    pub fn max_height(&self) -> usize {
        (0..BOARD_WIDTH)
            .map(|c| self.column_height(c))
            .max()
            .unwrap_or(0)
    }

    /// Count the number of holes (empty cells with at least one filled cell above).
    pub fn count_holes(&self) -> usize {
        let mut holes = 0;
        for col in 0..BOARD_WIDTH {
            let mut found_filled = false;
            for row in (0..BOARD_HEIGHT).rev() {
                if self.grid[row][col].is_occupied() {
                    found_filled = true;
                } else if found_filled {
                    holes += 1;
                }
            }
        }
        holes
    }

    /// Aggregate height: sum of all column heights.
    pub fn aggregate_height(&self) -> usize {
        (0..BOARD_WIDTH).map(|c| self.column_height(c)).sum()
    }

    /// Bumpiness: sum of absolute differences between adjacent column heights.
    pub fn bumpiness(&self) -> usize {
        let heights: Vec<usize> = (0..BOARD_WIDTH).map(|c| self.column_height(c)).collect();
        heights
            .windows(2)
            .map(|w| (w[0] as i32 - w[1] as i32).unsigned_abs() as usize)
            .sum()
    }

    /// Add garbage lines at the bottom with a gap at the given column.
    pub fn add_garbage(&mut self, count: usize, gap_col: usize) {
        // Shift everything up
        for row in (0..BOARD_HEIGHT).rev() {
            if row >= count {
                self.grid[row] = self.grid[row - count];
            } else {
                // Fill garbage row
                let mut garbage_row = [Cell::Garbage; BOARD_WIDTH];
                garbage_row[gap_col] = Cell::Empty;
                self.grid[row] = garbage_row;
            }
        }
    }

    /// Count garbage lines cleared from a set of cleared rows.
    pub fn count_garbage_in_rows(&self, rows: &[usize]) -> usize {
        rows.iter()
            .filter(|&&row| self.grid[row].iter().any(|c| matches!(c, Cell::Garbage)))
            .count()
    }

    /// Check if the piece would overlap with existing cells at spawn position.
    pub fn is_blocked(&self, piece: &Piece) -> bool {
        !self.piece_fits(piece)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        assert!(board.is_empty());
        assert_eq!(board.max_height(), 0);
        assert_eq!(board.count_holes(), 0);
    }

    #[test]
    fn test_piece_fits_empty_board() {
        let board = Board::new();
        let piece = Piece::new(PieceType::T);
        assert!(board.piece_fits(&piece));
    }

    #[test]
    fn test_collision_detection() {
        let mut board = Board::new();
        // Fill bottom row except one cell
        for col in 0..BOARD_WIDTH {
            board.set(col as i32, 0, Cell::Filled(PieceType::I));
        }

        let mut piece = Piece::new(PieceType::I);
        piece.y = 0;
        assert!(!board.piece_fits(&piece));
    }

    #[test]
    fn test_line_clear() {
        let mut board = Board::new();
        // Fill bottom row completely
        for col in 0..BOARD_WIDTH {
            board.set(col as i32, 0, Cell::Filled(PieceType::I));
        }
        let full = board.find_full_lines();
        assert_eq!(full, vec![0]);

        board.clear_lines(&full);
        // Bottom row should now be empty
        for col in 0..BOARD_WIDTH {
            assert!(board.get(col as i32, 0).is_empty());
        }
    }

    #[test]
    fn test_garbage_insertion() {
        let mut board = Board::new();
        board.set(5, 0, Cell::Filled(PieceType::T));
        board.add_garbage(2, 3);

        // The original piece should have moved up by 2
        assert!(board.get(5, 2).is_occupied());
        // Garbage rows at 0 and 1
        assert!(matches!(board.get(0, 0), Cell::Garbage));
        assert!(board.get(3, 0).is_empty()); // Gap
        assert!(matches!(board.get(0, 1), Cell::Garbage));
        assert!(board.get(3, 1).is_empty()); // Gap
    }

    #[test]
    fn test_column_height() {
        let mut board = Board::new();
        board.set(0, 0, Cell::Filled(PieceType::I));
        board.set(0, 1, Cell::Filled(PieceType::I));
        board.set(0, 2, Cell::Filled(PieceType::I));
        assert_eq!(board.column_height(0), 3);
        assert_eq!(board.column_height(1), 0);
    }

    #[test]
    fn test_holes_count() {
        let mut board = Board::new();
        // Create a hole: filled at row 1, empty at row 0
        board.set(0, 1, Cell::Filled(PieceType::I));
        assert_eq!(board.count_holes(), 1);
    }

    #[test]
    fn test_srs_rotation() {
        let board = Board::new();
        let piece = Piece::new(PieceType::T);
        // Should be able to rotate on empty board
        let result = board.try_rotate(&piece, RotationState::R1);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), (0, 0)); // No kick needed
    }
}
