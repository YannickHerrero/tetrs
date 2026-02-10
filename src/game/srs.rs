use super::piece::{PieceType, RotationState};

/// SRS wall kick offsets. Returns list of (dx, dy) to try.
/// Based on standard SRS + SRS+ 180° kicks.
pub fn get_kicks(
    piece_type: PieceType,
    from: RotationState,
    to: RotationState,
) -> &'static [(i32, i32)] {
    if piece_type == PieceType::O {
        return &[(0, 0)];
    }

    let is_180 = (from.index() as i32 - to.index() as i32).abs() == 2;

    if is_180 {
        if piece_type == PieceType::I {
            return get_i_180_kicks(from, to);
        } else {
            return get_normal_180_kicks(from, to);
        }
    }

    if piece_type == PieceType::I {
        get_i_kicks(from, to)
    } else {
        get_normal_kicks(from, to)
    }
}

/// Standard SRS kicks for J/L/S/T/Z pieces.
fn get_normal_kicks(from: RotationState, to: RotationState) -> &'static [(i32, i32)] {
    use RotationState::*;
    match (from, to) {
        (R0, R1) => &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        (R1, R0) => &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        (R1, R2) => &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        (R2, R1) => &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        (R2, R3) => &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
        (R3, R2) => &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
        (R3, R0) => &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
        (R0, R3) => &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
        _ => &[(0, 0)],
    }
}

/// SRS kicks for I piece.
fn get_i_kicks(from: RotationState, to: RotationState) -> &'static [(i32, i32)] {
    use RotationState::*;
    match (from, to) {
        (R0, R1) => &[(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
        (R1, R0) => &[(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
        (R1, R2) => &[(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
        (R2, R1) => &[(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
        (R2, R3) => &[(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
        (R3, R2) => &[(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
        (R3, R0) => &[(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
        (R0, R3) => &[(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
        _ => &[(0, 0)],
    }
}

/// SRS+ 180° kicks for normal pieces (from TETR.IO).
fn get_normal_180_kicks(from: RotationState, to: RotationState) -> &'static [(i32, i32)] {
    use RotationState::*;
    match (from, to) {
        (R0, R2) => &[(0, 0), (0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0)],
        (R2, R0) => &[(0, 0), (0, -1), (-1, -1), (1, -1), (-1, 0), (1, 0)],
        (R1, R3) => &[(0, 0), (1, 0), (1, 2), (1, 1), (0, 2), (0, 1)],
        (R3, R1) => &[(0, 0), (-1, 0), (-1, 2), (-1, 1), (0, 2), (0, 1)],
        _ => &[(0, 0)],
    }
}

/// SRS+ 180° kicks for I piece.
fn get_i_180_kicks(from: RotationState, to: RotationState) -> &'static [(i32, i32)] {
    use RotationState::*;
    match (from, to) {
        (R0, R2) => &[(0, 0), (-1, 0), (-2, 0), (1, 0), (2, 0), (0, 1)],
        (R2, R0) => &[(0, 0), (1, 0), (2, 0), (-1, 0), (-2, 0), (0, -1)],
        (R1, R3) => &[(0, 0), (0, 1), (0, 2), (0, -1), (0, -2), (-1, 0)],
        (R3, R1) => &[(0, 0), (0, 1), (0, 2), (0, -1), (0, -2), (1, 0)],
        _ => &[(0, 0)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_o_piece_no_kicks() {
        let kicks = get_kicks(PieceType::O, RotationState::R0, RotationState::R1);
        assert_eq!(kicks.len(), 1);
        assert_eq!(kicks[0], (0, 0));
    }

    #[test]
    fn test_normal_kicks_have_5_entries() {
        let kicks = get_kicks(PieceType::T, RotationState::R0, RotationState::R1);
        assert_eq!(kicks.len(), 5);
        assert_eq!(kicks[0], (0, 0)); // First kick is always (0,0)
    }

    #[test]
    fn test_i_kicks_have_5_entries() {
        let kicks = get_kicks(PieceType::I, RotationState::R0, RotationState::R1);
        assert_eq!(kicks.len(), 5);
    }

    #[test]
    fn test_180_kicks_have_6_entries() {
        let kicks = get_kicks(PieceType::T, RotationState::R0, RotationState::R2);
        assert_eq!(kicks.len(), 6);
    }
}
