use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Widget;

use crate::data::high_scores::HighScoreStore;
use crate::ui::theme;
use crate::ui::widgets::sidebar::format_number;

/// High scores screen.
pub struct HighScoresScreen<'a> {
    pub store: &'a HighScoreStore,
    pub selected_tab: usize,
}

const TABS: &[&str] = &["SPRINT", "ENDLESS", "VERSUS"];

impl<'a> Widget for HighScoresScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Clear background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                buf.set_string(x, y, " ", Style::default().bg(theme::BG_COLOR));
            }
        }

        let center_x = area.x + area.width / 2;
        let mut y = area.y + 2;

        // Title
        let title = "HIGH SCORES";
        let tx = center_x.saturating_sub(title.len() as u16 / 2);
        buf.set_string(tx, y, title, theme::title_style());
        y += 2;

        // Tabs
        let mut tab_x = center_x.saturating_sub(15);
        for (i, tab) in TABS.iter().enumerate() {
            let style = if i == self.selected_tab {
                Style::default()
                    .fg(ratatui::style::Color::Rgb(100, 220, 255))
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default().fg(theme::TEXT_DIM)
            };
            buf.set_string(tab_x, y, tab, style);
            tab_x += tab.len() as u16 + 3;
        }
        y += 2;

        // Separator
        let sep = "─────────────────────────────────────";
        let sx = center_x.saturating_sub(sep.len() as u16 / 2);
        buf.set_string(sx, y, sep, Style::default().fg(theme::PANEL_COLOR));
        y += 1;

        // Header
        match self.selected_tab {
            0 => {
                // Sprint: rank, time, lines, pieces, date
                let header = format!(
                    " {:<4} {:<12} {:<6} {:<6} {}",
                    "#", "TIME", "LINES", "PCS", "DATE"
                );
                let hx = center_x.saturating_sub(header.len() as u16 / 2);
                buf.set_string(hx, y, &header, theme::stat_label_style());
                y += 1;

                for (i, entry) in self.store.sprint.iter().enumerate() {
                    if y >= area.y + area.height - 2 {
                        break;
                    }
                    let rank = format!("{:>2}.", i + 1);
                    let time = format_time_ms(entry.time_ms);
                    let line = format!(
                        " {:<4} {:<12} {:<6} {:<6} {}",
                        rank,
                        time,
                        entry.lines,
                        entry.pieces,
                        entry.date.format("%Y-%m-%d")
                    );
                    let lx = center_x.saturating_sub(line.len() as u16 / 2);
                    let style = if i == 0 {
                        Style::default()
                            .fg(ratatui::style::Color::Rgb(255, 215, 60))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        theme::stat_value_style()
                    };
                    buf.set_string(lx, y, &line, style);
                    y += 1;
                }
            }
            1 => {
                // Endless: rank, score, level, lines, date
                let header = format!(
                    " {:<4} {:<12} {:<6} {:<6} {}",
                    "#", "SCORE", "LEVEL", "LINES", "DATE"
                );
                let hx = center_x.saturating_sub(header.len() as u16 / 2);
                buf.set_string(hx, y, &header, theme::stat_label_style());
                y += 1;

                for (i, entry) in self.store.endless.iter().enumerate() {
                    if y >= area.y + area.height - 2 {
                        break;
                    }
                    let rank = format!("{:>2}.", i + 1);
                    let line = format!(
                        " {:<4} {:<12} {:<6} {:<6} {}",
                        rank,
                        format_number(entry.score),
                        entry.level,
                        entry.lines,
                        entry.date.format("%Y-%m-%d")
                    );
                    let lx = center_x.saturating_sub(line.len() as u16 / 2);
                    let style = if i == 0 {
                        Style::default()
                            .fg(ratatui::style::Color::Rgb(255, 215, 60))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        theme::stat_value_style()
                    };
                    buf.set_string(lx, y, &line, style);
                    y += 1;
                }
            }
            2 => {
                // Versus: rank, result, difficulty, damage, time, date
                let header = format!(
                    " {:<4} {:<6} {:<8} {:<6} {:<10} {}",
                    "#", "RESULT", "DIFF", "DMG", "TIME", "DATE"
                );
                let hx = center_x.saturating_sub(header.len() as u16 / 2);
                buf.set_string(hx, y, &header, theme::stat_label_style());
                y += 1;

                for (i, entry) in self.store.versus.iter().enumerate() {
                    if y >= area.y + area.height - 2 {
                        break;
                    }
                    let rank = format!("{:>2}.", i + 1);
                    let result_str = if entry.won { "WIN" } else { "LOSS" };
                    let time = format_time_ms(entry.time_ms);
                    let line = format!(
                        " {:<4} {:<6} {:<8} {:<6} {:<10} {}",
                        rank,
                        result_str,
                        entry.difficulty,
                        entry.damage_sent,
                        time,
                        entry.date.format("%Y-%m-%d")
                    );
                    let lx = center_x.saturating_sub(line.len() as u16 / 2);
                    let result_color = if entry.won {
                        ratatui::style::Color::Rgb(80, 255, 120)
                    } else {
                        ratatui::style::Color::Rgb(255, 80, 80)
                    };
                    let style = if i == 0 {
                        Style::default()
                            .fg(ratatui::style::Color::Rgb(255, 215, 60))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(result_color)
                    };
                    buf.set_string(lx, y, &line, style);
                    y += 1;
                }
            }
            _ => {}
        }

        // Show empty message if no scores
        let is_empty = match self.selected_tab {
            0 => self.store.sprint.is_empty(),
            1 => self.store.endless.is_empty(),
            2 => self.store.versus.is_empty(),
            _ => true,
        };
        if is_empty && y < area.y + area.height - 3 {
            let msg = "No scores yet. Play some games!";
            let mx = center_x.saturating_sub(msg.len() as u16 / 2);
            buf.set_string(mx, y + 2, msg, theme::menu_desc_style());
        }

        // Controls
        let controls = "h/l: switch tab   Esc: back";
        let cx = center_x.saturating_sub(controls.len() as u16 / 2);
        let cy = area.y + area.height - 2;
        buf.set_string(cx, cy, controls, theme::menu_desc_style());
    }
}

fn format_time_ms(ms: u64) -> String {
    let minutes = ms / 60_000;
    let seconds = (ms % 60_000) / 1000;
    let millis = ms % 1000;
    format!("{:02}:{:02}.{:03}", minutes, seconds, millis)
}
