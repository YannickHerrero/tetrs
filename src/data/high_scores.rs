use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const MAX_SCORES: usize = 10;

/// Sprint high score entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprintEntry {
    pub time_ms: u64,
    pub lines: u32,
    pub pieces: u32,
    pub date: DateTime<Utc>,
}

/// Endless high score entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndlessEntry {
    pub score: u64,
    pub level: u32,
    pub lines: u32,
    pub date: DateTime<Utc>,
}

/// Versus high score entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersusEntry {
    pub won: bool,
    pub difficulty: String,
    pub time_ms: u64,
    pub damage_sent: u32,
    pub date: DateTime<Utc>,
}

/// All high scores.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighScoreStore {
    pub sprint: Vec<SprintEntry>,
    pub endless: Vec<EndlessEntry>,
    pub versus: Vec<VersusEntry>,
}

impl HighScoreStore {
    /// Get the config directory path.
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("tetrs"))
    }

    /// Get the high scores file path.
    fn file_path() -> Option<PathBuf> {
        Self::config_path().map(|d| d.join("high_scores.json"))
    }

    /// Load from disk, or create empty if not found.
    pub fn load() -> Self {
        let path = match Self::file_path() {
            Some(p) => p,
            None => return Self::default(),
        };

        match fs::read_to_string(&path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save to disk.
    pub fn save(&self) {
        let path = match Self::file_path() {
            Some(p) => p,
            None => return,
        };

        // Ensure directory exists
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }

        // Write to temp file then rename for atomicity
        let temp_path = path.with_extension("tmp");
        if let Ok(data) = serde_json::to_string_pretty(self) {
            if fs::write(&temp_path, &data).is_ok() {
                let _ = fs::rename(&temp_path, &path);
            }
        }
    }

    /// Add a sprint result. Returns true if it's a new high score.
    pub fn add_sprint(&mut self, time_ms: u64, lines: u32, pieces: u32) -> bool {
        let entry = SprintEntry {
            time_ms,
            lines,
            pieces,
            date: Utc::now(),
        };

        let is_best = self.sprint.first().map_or(true, |e| time_ms < e.time_ms);

        self.sprint.push(entry);
        self.sprint.sort_by_key(|e| e.time_ms);
        self.sprint.truncate(MAX_SCORES);
        self.save();

        is_best
    }

    /// Add an endless result. Returns true if it's a new high score.
    pub fn add_endless(&mut self, score: u64, level: u32, lines: u32) -> bool {
        let entry = EndlessEntry {
            score,
            level,
            lines,
            date: Utc::now(),
        };

        let is_best = self.endless.first().map_or(true, |e| score > e.score);

        self.endless.push(entry);
        self.endless.sort_by(|a, b| b.score.cmp(&a.score));
        self.endless.truncate(MAX_SCORES);
        self.save();

        is_best
    }

    /// Add a versus result. Returns true if it's a new top entry.
    pub fn add_versus(
        &mut self,
        won: bool,
        difficulty: &str,
        time_ms: u64,
        damage_sent: u32,
    ) -> bool {
        let entry = VersusEntry {
            won,
            difficulty: difficulty.to_string(),
            time_ms,
            damage_sent,
            date: Utc::now(),
        };

        let is_best = self.versus.first().map_or(true, |e| won && !e.won);

        self.versus.push(entry);
        // Sort: wins first, then by damage sent
        self.versus
            .sort_by(|a, b| b.won.cmp(&a.won).then(b.damage_sent.cmp(&a.damage_sent)));
        self.versus.truncate(MAX_SCORES);
        self.save();

        is_best
    }
}
