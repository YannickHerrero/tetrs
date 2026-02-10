use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Widget;

use crate::game::stats::Stats;
use crate::ui::theme;
use crate::ui::widgets::sidebar::format_number;

/// Result to display on game over.
#[derive(Debug, Clone)]
pub struct GameResult {
    pub mode_name: String,
    pub primary_label: String,
    pub primary_value: String,
    pub is_new_high_score: bool,
    pub stats: Stats,
    pub won: Option<bool>, // For versus: Some(true/false), else None
}

/// Game over screen widget.
pub struct GameOverScreen<'a> {
    pub result: &'a GameResult,
    pub frame: u64,
}

impl<'a> Widget for GameOverScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Clear background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                buf.set_string(x, y, " ", Style::default().bg(theme::BG_COLOR));
            }
        }

        let center_x = area.x + area.width / 2;
        let mut y = area.y + area.height / 6;

        // Title
        let title = if let Some(won) = self.result.won {
            if won {
                "VICTORY!"
            } else {
                "DEFEAT"
            }
        } else {
            "GAME OVER"
        };
        let title_color = if let Some(won) = self.result.won {
            if won {
                ratatui::style::Color::Rgb(80, 255, 120)
            } else {
                ratatui::style::Color::Rgb(255, 60, 60)
            }
        } else {
            ratatui::style::Color::Rgb(255, 60, 60)
        };

        let title_style = Style::default()
            .fg(title_color)
            .add_modifier(Modifier::BOLD);
        let tx = center_x.saturating_sub(title.len() as u16 / 2);
        buf.set_string(tx, y, title, title_style);
        y += 2;

        // Mode name
        let mode_style = Style::default().fg(theme::TEXT_DIM);
        let mx = center_x.saturating_sub(self.result.mode_name.len() as u16 / 2);
        buf.set_string(mx, y, &self.result.mode_name, mode_style);
        y += 2;

        // Primary result (highlighted)
        let primary_label = &self.result.primary_label;
        let primary_value = &self.result.primary_value;
        let plx = center_x.saturating_sub(primary_label.len() as u16 / 2);
        buf.set_string(plx, y, primary_label, theme::stat_label_style());
        y += 1;
        let pvx = center_x.saturating_sub(primary_value.len() as u16 / 2);
        let pv_style = Style::default()
            .fg(ratatui::style::Color::Rgb(100, 220, 255))
            .add_modifier(Modifier::BOLD);
        buf.set_string(pvx, y, primary_value, pv_style);
        y += 1;

        // New high score indicator
        if self.result.is_new_high_score {
            y += 1;
            let hs_text = "★ NEW HIGH SCORE ★";
            let hsx = center_x.saturating_sub(hs_text.len() as u16 / 2);
            let phase = (self.frame as f32 * 0.1).sin() * 0.5 + 0.5;
            let hs_color = ratatui::style::Color::Rgb(
                (255.0 * phase) as u8,
                (215.0 * phase + 40.0 * (1.0 - phase)) as u8,
                (60.0 * phase + 200.0 * (1.0 - phase)) as u8,
            );
            buf.set_string(
                hsx,
                y,
                hs_text,
                Style::default().fg(hs_color).add_modifier(Modifier::BOLD),
            );
            y += 1;
        }

        y += 1;

        // Separator
        let sep = "─────────────────────────";
        let sx = center_x.saturating_sub(sep.len() as u16 / 2);
        buf.set_string(sx, y, sep, Style::default().fg(theme::PANEL_COLOR));
        y += 2;

        // Stats
        let stats_data = [
            ("Score", format_number(self.result.stats.score)),
            ("Lines", self.result.stats.lines_cleared.to_string()),
            ("Level", self.result.stats.level.to_string()),
            ("Pieces", self.result.stats.pieces_placed.to_string()),
            ("Time", self.result.stats.format_time()),
            ("PPS", format!("{:.2}", self.result.stats.pps())),
            ("APM", format!("{:.1}", self.result.stats.apm())),
            ("Max Combo", self.result.stats.max_combo.to_string()),
            ("Max B2B", self.result.stats.max_btb.to_string()),
            ("Quads", self.result.stats.quads.to_string()),
            ("T-Spins", self.result.stats.tspins.to_string()),
            ("PCs", self.result.stats.perfect_clears.to_string()),
        ];

        for (label, value) in &stats_data {
            if y + 1 >= area.y + area.height - 2 {
                break;
            }
            let stat_str = format!("{:<12} {}", label, value);
            let stat_x = center_x.saturating_sub(stat_str.len() as u16 / 2);
            buf.set_string(stat_x, y, label, theme::stat_label_style());
            buf.set_string(stat_x + 13, y, value, theme::stat_value_style());
            y += 1;
        }

        // Controls
        let controls = "[R] Restart   [Esc] Menu   [Q] Quit";
        let cx = center_x.saturating_sub(controls.len() as u16 / 2);
        let cy = area.y + area.height - 2;
        if cy > y + 1 {
            buf.set_string(cx, cy, controls, theme::menu_desc_style());
        }
    }
}
