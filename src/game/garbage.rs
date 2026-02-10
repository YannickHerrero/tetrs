use rand::Rng;
use std::time::Duration;

/// A pending garbage batch with travel time.
#[derive(Debug, Clone)]
struct GarbageBatch {
    lines: u32,
    time_remaining: Duration,
}

/// Garbage queue: manages incoming garbage with travel time and cancellation.
#[derive(Debug, Clone)]
pub struct GarbageQueue {
    queue: Vec<GarbageBatch>,
    /// Travel time for new garbage (default 500ms).
    pub travel_time: Duration,
    /// Garbage messiness: 0.0 = same column every line, 1.0 = random column.
    pub messiness: f64,
    /// Last gap column used.
    last_gap: usize,
}

impl GarbageQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            travel_time: Duration::from_millis(500),
            messiness: 0.3,
            last_gap: 4,
        }
    }

    /// Add garbage to the queue.
    pub fn add(&mut self, lines: u32) {
        if lines > 0 {
            self.queue.push(GarbageBatch {
                lines,
                time_remaining: self.travel_time,
            });
        }
    }

    /// Cancel incoming garbage with attack damage. Returns remaining attack after cancellation.
    pub fn cancel(&mut self, mut attack: u32) -> u32 {
        while attack > 0 && !self.queue.is_empty() {
            let batch = &mut self.queue[0];
            if attack >= batch.lines {
                attack -= batch.lines;
                self.queue.remove(0);
            } else {
                batch.lines -= attack;
                attack = 0;
            }
        }
        attack
    }

    /// Tick all garbage timers. Returns total lines of garbage ready to deploy.
    pub fn tick(&mut self, dt: Duration) -> u32 {
        let mut ready = 0;
        self.queue.retain_mut(|batch| {
            if dt >= batch.time_remaining {
                ready += batch.lines;
                false
            } else {
                batch.time_remaining -= dt;
                true
            }
        });
        ready
    }

    /// Get a gap column for a garbage line.
    pub fn gap_column<R: Rng>(&mut self, rng: &mut R) -> usize {
        if rng.gen::<f64>() < self.messiness {
            self.last_gap = rng.gen_range(0..10);
        }
        self.last_gap
    }

    /// Total pending garbage lines.
    pub fn pending(&self) -> u32 {
        self.queue.iter().map(|b| b.lines).sum()
    }

    /// Clear all pending garbage.
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_garbage() {
        let mut gq = GarbageQueue::new();
        gq.add(4);
        assert_eq!(gq.pending(), 4);
    }

    #[test]
    fn test_cancel_garbage() {
        let mut gq = GarbageQueue::new();
        gq.add(4);
        let remaining = gq.cancel(2);
        assert_eq!(remaining, 0);
        assert_eq!(gq.pending(), 2);
    }

    #[test]
    fn test_cancel_overflow() {
        let mut gq = GarbageQueue::new();
        gq.add(2);
        let remaining = gq.cancel(5);
        assert_eq!(remaining, 3);
        assert_eq!(gq.pending(), 0);
    }

    #[test]
    fn test_tick_garbage() {
        let mut gq = GarbageQueue::new();
        gq.add(3);
        // Not ready yet
        let ready = gq.tick(Duration::from_millis(200));
        assert_eq!(ready, 0);
        // Now ready
        let ready = gq.tick(Duration::from_millis(400));
        assert_eq!(ready, 3);
    }
}
