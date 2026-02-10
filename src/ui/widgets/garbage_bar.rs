use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

use crate::game::board::VISIBLE_HEIGHT;
use crate::ui::theme;

/// Widget that renders the incoming garbage indicator bar.
pub struct GarbageBarWidget {
    /// Number of pending garbage lines.
    pub pending: u32,
}

impl Widget for GarbageBarWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 1 || area.height < 2 {
            return;
        }

        let x = area.x;
        let bar_height = area.height.min(VISIBLE_HEIGHT as u16);

        // Draw the bar from bottom to top
        let filled = (self.pending as u16).min(bar_height);

        for row in 0..bar_height {
            let y = area.y + area.height.saturating_sub(1) - row;
            if row < filled {
                // Color intensity increases with more garbage
                let color = if self.pending >= 8 {
                    theme::DANGER_COLOR
                } else {
                    theme::GARBAGE_BAR_COLOR
                };
                buf.set_string(x, y, "▐", Style::default().fg(color));
            } else {
                buf.set_string(x, y, "▐", Style::default().fg(theme::GARBAGE_BAR_BG));
            }
        }
    }
}
