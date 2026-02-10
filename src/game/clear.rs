use super::piece::PieceType;

/// Type of line clear or spin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClearType {
    None,
    Single,
    Double,
    Triple,
    Quad,
    TSpin, // T-spin with 0 lines
    TSpinSingle,
    TSpinDouble,
    TSpinTriple,
    MiniTSpin, // Mini T-spin with 0 lines
    MiniTSpinSingle,
    MiniTSpinDouble,
    AllSpin(u32), // All-spin with N lines
}

impl ClearType {
    /// Determine clear type from the number of lines, spin state, and piece type.
    pub fn classify(lines: u32, spin: SpinType, _piece_type: PieceType) -> Self {
        match spin {
            SpinType::TSpin => match lines {
                0 => ClearType::TSpin,
                1 => ClearType::TSpinSingle,
                2 => ClearType::TSpinDouble,
                3 => ClearType::TSpinTriple,
                _ => ClearType::Quad, // Shouldn't happen for T
            },
            SpinType::MiniTSpin => match lines {
                0 => ClearType::MiniTSpin,
                1 => ClearType::MiniTSpinSingle,
                2 => ClearType::MiniTSpinDouble,
                _ => ClearType::Triple,
            },
            SpinType::AllSpin => {
                if lines > 0 {
                    ClearType::AllSpin(lines)
                } else {
                    ClearType::None
                }
            }
            SpinType::None => match lines {
                0 => ClearType::None,
                1 => ClearType::Single,
                2 => ClearType::Double,
                3 => ClearType::Triple,
                4 => ClearType::Quad,
                _ => ClearType::Quad,
            },
        }
    }

    /// Is this a "difficult" clear (for BTB tracking)?
    pub fn is_difficult(&self) -> bool {
        matches!(
            self,
            ClearType::Quad
                | ClearType::TSpin
                | ClearType::TSpinSingle
                | ClearType::TSpinDouble
                | ClearType::TSpinTriple
                | ClearType::MiniTSpin
                | ClearType::MiniTSpinSingle
                | ClearType::MiniTSpinDouble
                | ClearType::AllSpin(_)
        )
    }

    /// Display name for the clear type.
    pub fn display_name(&self) -> &'static str {
        match self {
            ClearType::None => "",
            ClearType::Single => "SINGLE",
            ClearType::Double => "DOUBLE",
            ClearType::Triple => "TRIPLE",
            ClearType::Quad => "TETRIS",
            ClearType::TSpin => "T-SPIN",
            ClearType::TSpinSingle => "T-SPIN SINGLE",
            ClearType::TSpinDouble => "T-SPIN DOUBLE",
            ClearType::TSpinTriple => "T-SPIN TRIPLE",
            ClearType::MiniTSpin => "MINI T-SPIN",
            ClearType::MiniTSpinSingle => "MINI T-SPIN SINGLE",
            ClearType::MiniTSpinDouble => "MINI T-SPIN DOUBLE",
            ClearType::AllSpin(n) => match n {
                1 => "ALL-SPIN SINGLE",
                2 => "ALL-SPIN DOUBLE",
                3 => "ALL-SPIN TRIPLE",
                _ => "ALL-SPIN",
            },
        }
    }

    /// Number of lines this clear represents.
    pub fn lines(&self) -> u32 {
        match self {
            ClearType::None | ClearType::TSpin | ClearType::MiniTSpin => 0,
            ClearType::Single | ClearType::TSpinSingle | ClearType::MiniTSpinSingle => 1,
            ClearType::Double | ClearType::TSpinDouble | ClearType::MiniTSpinDouble => 2,
            ClearType::Triple | ClearType::TSpinTriple => 3,
            ClearType::Quad => 4,
            ClearType::AllSpin(n) => *n,
        }
    }
}

/// Spin detection result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinType {
    None,
    TSpin,
    MiniTSpin,
    AllSpin,
}

impl std::fmt::Display for ClearType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_single() {
        let ct = ClearType::classify(1, SpinType::None, PieceType::T);
        assert_eq!(ct, ClearType::Single);
        assert!(!ct.is_difficult());
    }

    #[test]
    fn test_classify_tetris() {
        let ct = ClearType::classify(4, SpinType::None, PieceType::I);
        assert_eq!(ct, ClearType::Quad);
        assert!(ct.is_difficult());
    }

    #[test]
    fn test_classify_tspin_double() {
        let ct = ClearType::classify(2, SpinType::TSpin, PieceType::T);
        assert_eq!(ct, ClearType::TSpinDouble);
        assert!(ct.is_difficult());
    }

    #[test]
    fn test_classify_mini_tspin() {
        let ct = ClearType::classify(1, SpinType::MiniTSpin, PieceType::T);
        assert_eq!(ct, ClearType::MiniTSpinSingle);
        assert!(ct.is_difficult());
    }
}
