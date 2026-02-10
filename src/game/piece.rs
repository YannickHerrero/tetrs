use ratatui::style::Color;

/// The 7 standard tetromino types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl PieceType {
    pub const ALL: [PieceType; 7] = [
        PieceType::I,
        PieceType::O,
        PieceType::T,
        PieceType::S,
        PieceType::Z,
        PieceType::J,
        PieceType::L,
    ];

    pub fn color(self) -> Color {
        match self {
            PieceType::I => Color::Rgb(101, 219, 200), // Cyan
            PieceType::O => Color::Rgb(242, 215, 76),  // Yellow
            PieceType::T => Color::Rgb(193, 50, 208),  // Purple
            PieceType::S => Color::Rgb(122, 205, 68),  // Green
            PieceType::Z => Color::Rgb(216, 58, 40),   // Red
            PieceType::J => Color::Rgb(51, 88, 221),   // Blue
            PieceType::L => Color::Rgb(237, 169, 63),  // Orange
        }
    }

    pub fn bright_color(self) -> Color {
        match self {
            PieceType::I => Color::Rgb(140, 240, 225),
            PieceType::O => Color::Rgb(255, 235, 120),
            PieceType::T => Color::Rgb(225, 100, 240),
            PieceType::S => Color::Rgb(160, 235, 110),
            PieceType::Z => Color::Rgb(245, 100, 85),
            PieceType::J => Color::Rgb(100, 135, 245),
            PieceType::L => Color::Rgb(255, 200, 110),
        }
    }

    pub fn dim_color(self) -> Color {
        match self {
            PieceType::I => Color::Rgb(50, 110, 100),
            PieceType::O => Color::Rgb(121, 107, 38),
            PieceType::T => Color::Rgb(96, 25, 104),
            PieceType::S => Color::Rgb(61, 102, 34),
            PieceType::Z => Color::Rgb(108, 29, 20),
            PieceType::J => Color::Rgb(25, 44, 110),
            PieceType::L => Color::Rgb(118, 84, 31),
        }
    }
}

/// Rotation state of a piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RotationState {
    /// Spawn state (0)
    R0,
    /// Clockwise (R, 1)
    R1,
    /// 180 (2)
    R2,
    /// Counter-clockwise (L, 3)
    R3,
}

impl RotationState {
    pub fn cw(self) -> Self {
        match self {
            RotationState::R0 => RotationState::R1,
            RotationState::R1 => RotationState::R2,
            RotationState::R2 => RotationState::R3,
            RotationState::R3 => RotationState::R0,
        }
    }

    pub fn ccw(self) -> Self {
        match self {
            RotationState::R0 => RotationState::R3,
            RotationState::R1 => RotationState::R0,
            RotationState::R2 => RotationState::R1,
            RotationState::R3 => RotationState::R2,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            RotationState::R0 => RotationState::R2,
            RotationState::R1 => RotationState::R3,
            RotationState::R2 => RotationState::R0,
            RotationState::R3 => RotationState::R1,
        }
    }

    pub fn index(self) -> usize {
        match self {
            RotationState::R0 => 0,
            RotationState::R1 => 1,
            RotationState::R2 => 2,
            RotationState::R3 => 3,
        }
    }
}

/// Active piece on the board.
#[derive(Debug, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub rotation: RotationState,
    /// Position of piece origin (col, row). Row 0 = bottom, row 19 = top visible.
    pub x: i32,
    pub y: i32,
}

impl Piece {
    pub fn new(piece_type: PieceType) -> Self {
        Self {
            piece_type,
            rotation: RotationState::R0,
            x: 3,
            y: 20, // Spawn above visible area (rows 0-19 visible, 20-39 buffer)
        }
    }

    /// Get the 4 cell positions (col, row) for the current state.
    pub fn cells(&self) -> [(i32, i32); 4] {
        let offsets = get_cells(self.piece_type, self.rotation);
        [
            (self.x + offsets[0].0, self.y + offsets[0].1),
            (self.x + offsets[1].0, self.y + offsets[1].1),
            (self.x + offsets[2].0, self.y + offsets[2].1),
            (self.x + offsets[3].0, self.y + offsets[3].1),
        ]
    }

    /// Get cells at a given position and rotation (for testing placements).
    pub fn cells_at(&self, x: i32, y: i32, rotation: RotationState) -> [(i32, i32); 4] {
        let offsets = get_cells(self.piece_type, rotation);
        [
            (x + offsets[0].0, y + offsets[0].1),
            (x + offsets[1].0, y + offsets[1].1),
            (x + offsets[2].0, y + offsets[2].1),
            (x + offsets[3].0, y + offsets[3].1),
        ]
    }
}

/// Get the 4 cell offsets for a piece type and rotation state.
/// Offsets are relative to the piece origin.
pub fn get_cells(piece_type: PieceType, rotation: RotationState) -> [(i32, i32); 4] {
    match piece_type {
        PieceType::I => I_CELLS[rotation.index()],
        PieceType::O => O_CELLS[rotation.index()],
        PieceType::T => T_CELLS[rotation.index()],
        PieceType::S => S_CELLS[rotation.index()],
        PieceType::Z => Z_CELLS[rotation.index()],
        PieceType::J => J_CELLS[rotation.index()],
        PieceType::L => L_CELLS[rotation.index()],
    }
}

// Piece cell offsets for all rotation states.
// Using SRS standard orientations. (col_offset, row_offset) from origin.
// Row increases upward, col increases rightward.

const I_CELLS: [[(i32, i32); 4]; 4] = [
    // R0 (spawn): horizontal, middle row
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    // R1 (CW): vertical, right column
    [(2, 1), (2, 0), (2, -1), (2, -2)],
    // R2 (180): horizontal, lower row
    [(0, -1), (1, -1), (2, -1), (3, -1)],
    // R3 (CCW): vertical, left column
    [(1, 1), (1, 0), (1, -1), (1, -2)],
];

const O_CELLS: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
];

const T_CELLS: [[(i32, i32); 4]; 4] = [
    // R0: T pointing up
    [(0, 0), (1, 0), (2, 0), (1, 1)],
    // R1: T pointing right
    [(1, 1), (1, 0), (1, -1), (2, 0)],
    // R2: T pointing down
    [(0, 0), (1, 0), (2, 0), (1, -1)],
    // R3: T pointing left
    [(1, 1), (1, 0), (1, -1), (0, 0)],
];

const S_CELLS: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (1, 1), (2, 1)],
    [(1, 1), (1, 0), (2, 0), (2, -1)],
    [(0, -1), (1, -1), (1, 0), (2, 0)],
    [(0, 1), (0, 0), (1, 0), (1, -1)],
];

const Z_CELLS: [[(i32, i32); 4]; 4] = [
    [(0, 1), (1, 1), (1, 0), (2, 0)],
    [(2, 1), (2, 0), (1, 0), (1, -1)],
    [(0, 0), (1, 0), (1, -1), (2, -1)],
    [(1, 1), (1, 0), (0, 0), (0, -1)],
];

const J_CELLS: [[(i32, i32); 4]; 4] = [
    [(0, 1), (0, 0), (1, 0), (2, 0)],
    [(1, 1), (2, 1), (1, 0), (1, -1)],
    [(0, 0), (1, 0), (2, 0), (2, -1)],
    [(1, 1), (1, 0), (0, -1), (1, -1)],
];

const L_CELLS: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (2, 0), (2, 1)],
    [(1, 1), (1, 0), (1, -1), (2, -1)],
    [(0, -1), (0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (1, 0), (1, -1)],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_spawn() {
        let piece = Piece::new(PieceType::T);
        assert_eq!(piece.x, 3);
        assert_eq!(piece.y, 20);
        assert_eq!(piece.rotation, RotationState::R0);
    }

    #[test]
    fn test_rotation_cycle() {
        let r = RotationState::R0;
        assert_eq!(r.cw().cw().cw().cw(), RotationState::R0);
        assert_eq!(r.ccw().ccw().ccw().ccw(), RotationState::R0);
        assert_eq!(r.cw(), r.ccw().flip());
    }

    #[test]
    fn test_piece_cells() {
        let piece = Piece::new(PieceType::I);
        let cells = piece.cells();
        // I piece at spawn: horizontal at row 20
        assert_eq!(cells[0], (3, 20));
        assert_eq!(cells[1], (4, 20));
        assert_eq!(cells[2], (5, 20));
        assert_eq!(cells[3], (6, 20));
    }

    #[test]
    fn test_o_piece_rotation_invariant() {
        // O piece should have same cells in all rotations
        for i in 0..4 {
            assert_eq!(O_CELLS[i], O_CELLS[0]);
        }
    }
}
