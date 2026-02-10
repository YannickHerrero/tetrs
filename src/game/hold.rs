use super::piece::PieceType;

/// Hold piece state.
#[derive(Debug, Clone)]
pub struct Hold {
    pub piece: Option<PieceType>,
    pub used_this_turn: bool,
}

impl Hold {
    pub fn new() -> Self {
        Self {
            piece: None,
            used_this_turn: false,
        }
    }

    /// Attempt to hold the current piece. Returns the previously held piece (if any).
    /// Returns None if hold was already used this turn.
    pub fn hold(&mut self, current: PieceType) -> Result<Option<PieceType>, ()> {
        if self.used_this_turn {
            return Err(());
        }
        self.used_this_turn = true;
        let prev = self.piece;
        self.piece = Some(current);
        Ok(prev)
    }

    /// Reset the hold-used flag (call when a new piece locks).
    pub fn reset_turn(&mut self) {
        self.used_this_turn = false;
    }

    /// Full reset (new game).
    pub fn reset(&mut self) {
        self.piece = None;
        self.used_this_turn = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_hold() {
        let mut hold = Hold::new();
        assert!(hold.piece.is_none());
        let result = hold.hold(PieceType::T);
        assert_eq!(result, Ok(None));
        assert_eq!(hold.piece, Some(PieceType::T));
    }

    #[test]
    fn test_swap_hold() {
        let mut hold = Hold::new();
        hold.hold(PieceType::T).unwrap();
        hold.reset_turn();
        let result = hold.hold(PieceType::I);
        assert_eq!(result, Ok(Some(PieceType::T)));
        assert_eq!(hold.piece, Some(PieceType::I));
    }

    #[test]
    fn test_double_hold_blocked() {
        let mut hold = Hold::new();
        hold.hold(PieceType::T).unwrap();
        assert!(hold.hold(PieceType::I).is_err());
    }
}
