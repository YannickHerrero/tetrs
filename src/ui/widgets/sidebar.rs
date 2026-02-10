use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

use crate::game::scoring::Scoring;
use crate::game::stats::Stats;
use crate::ui::theme;

/// Widget that renders the score/stats sidebar panel.
pub struct SidebarWidget<'a> {
    pub scoring: &'a Scoring,
    pub stats: &'a Stats,
    pub title: &'a str,
}

impl<'a> Widget for SidebarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 10 || area.height < 12 {
            return;
        }

        let x = area.x;
        let mut y = area.y;

        // Title
        let title = format!(" {} ", self.title);
        buf.set_string(x, y, &title, theme::header_style());
        y += 1;

        // Separator
        let sep: String = theme::PANEL_H.repeat(area.width as usize);
        buf.set_string(x, y, &sep, Style::default().fg(theme::PANEL_COLOR));
        y += 1;

        // Score
        self.draw_stat(
            buf,
            x,
            y,
            area.width,
            "SCORE",
            &format_number(self.scoring.score),
        );
        y += 2;

        // Level
        let level_style = Style::default().fg(theme::level_color(self.scoring.level));
        buf.set_string(x + 1, y, "LEVEL", theme::stat_label_style());
        y += 1;
        let level_str = format!("{}", self.scoring.level);
        buf.set_string(x + 1, y, &level_str, level_style);
        y += 2;

        // Lines
        self.draw_stat(
            buf,
            x,
            y,
            area.width,
            "LINES",
            &self.scoring.lines_cleared.to_string(),
        );
        y += 2;

        // Time
        self.draw_stat(buf, x, y, area.width, "TIME", &self.stats.format_time());
        y += 2;

        // PPS
        if y + 1 < area.y + area.height {
            self.draw_stat(
                buf,
                x,
                y,
                area.width,
                "PPS",
                &format!("{:.2}", self.stats.pps()),
            );
            y += 2;
        }

        // Combo
        if y + 1 < area.y + area.height && self.scoring.combo >= 0 {
            let combo_str = format!("{}", self.scoring.combo);
            let combo_style = Style::default().fg(theme::combo_color(self.scoring.combo as u32));
            buf.set_string(x + 1, y, "COMBO", theme::stat_label_style());
            y += 1;
            buf.set_string(x + 1, y, &combo_str, combo_style);
            y += 2;
        }

        // BTB
        if y + 1 < area.y + area.height && self.scoring.btb > 0 {
            let btb_str = format!("B2B ×{}", self.scoring.btb);
            let btb_style = Style::default().fg(theme::btb_color(self.scoring.btb as u32));
            buf.set_string(x + 1, y, &btb_str, btb_style);
        }
    }
}

impl<'a> SidebarWidget<'a> {
    fn draw_stat(&self, buf: &mut Buffer, x: u16, y: u16, _width: u16, label: &str, value: &str) {
        buf.set_string(x + 1, y, label, theme::stat_label_style());
        buf.set_string(x + 1, y + 1, value, theme::stat_value_style());
    }
}

/// Format a number with thousands separators.
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
