use rand::seq::SliceRandom;
use rand::Rng;

use super::piece::PieceType;

/// 7-bag randomizer with double-bag for seamless preview.
#[derive(Debug, Clone)]
pub struct Bag {
    current: Vec<PieceType>,
    next: Vec<PieceType>,
    index: usize,
}

impl Bag {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let mut current = Self::new_bag(rng);
        let next = Self::new_bag(rng);
        current.reverse(); // We'll pop from the end
        Self {
            current,
            next,
            index: 0,
        }
    }

    fn new_bag<R: Rng>(rng: &mut R) -> Vec<PieceType> {
        let mut bag = PieceType::ALL.to_vec();
        bag.shuffle(rng);
        bag
    }

    /// Draw the next piece from the bag.
    pub fn next<R: Rng>(&mut self, rng: &mut R) -> PieceType {
        self.index += 1;
        if let Some(piece) = self.current.pop() {
            piece
        } else {
            // Current bag exhausted, swap
            std::mem::swap(&mut self.current, &mut self.next);
            self.current.reverse();
            self.next = Self::new_bag(rng);
            self.current.pop().unwrap()
        }
    }

    /// Peek at upcoming pieces (0 = next piece, 1 = one after, etc.)
    pub fn peek(&self, count: usize) -> Vec<PieceType> {
        let mut result = Vec::with_capacity(count);
        let current_len = self.current.len();

        for i in 0..count {
            if i < current_len {
                // Read from current bag (reversed, so end is next)
                result.push(self.current[current_len - 1 - i]);
            } else {
                // Read from next bag
                let next_idx = i - current_len;
                if next_idx < self.next.len() {
                    result.push(self.next[next_idx]);
                }
            }
        }
        result
    }

    /// How many pieces have been drawn total.
    pub fn pieces_drawn(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_bag_produces_all_pieces() {
        let mut rng = thread_rng();
        let mut bag = Bag::new(&mut rng);
        let mut seen = std::collections::HashSet::new();
        for _ in 0..7 {
            seen.insert(bag.next(&mut rng));
        }
        assert_eq!(seen.len(), 7);
    }

    #[test]
    fn test_bag_peek() {
        let mut rng = thread_rng();
        let bag = Bag::new(&mut rng);
        let preview = bag.peek(3);
        assert_eq!(preview.len(), 3);
    }

    #[test]
    fn test_bag_peek_matches_next() {
        let mut rng = thread_rng();
        let mut bag = Bag::new(&mut rng);
        let preview = bag.peek(3);
        let first = bag.next(&mut rng);
        assert_eq!(preview[0], first);
    }

    #[test]
    fn test_bag_crosses_boundary() {
        let mut rng = thread_rng();
        let mut bag = Bag::new(&mut rng);
        // Draw 14 pieces (2 full bags)
        for _ in 0..14 {
            bag.next(&mut rng);
        }
        // Should still produce valid pieces
        let piece = bag.next(&mut rng);
        assert!(PieceType::ALL.contains(&piece));
    }
}
