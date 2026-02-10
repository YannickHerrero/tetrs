use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Widget;

use crate::game::clear::ClearType;
use crate::game::scoring::Scoring;
use crate::ui::theme;

/// Widget that shows the last clear type, combo, and BTB text.
pub struct ActionTextWidget<'a> {
    pub clear_type: Option<&'a ClearType>,
    pub scoring: &'a Scoring,
    pub opacity: f32,
}

impl<'a> Widget for ActionTextWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.opacity <= 0.01 || area.width < 10 || area.height < 1 {
            return;
        }

        let x = area.x;
        let mut y = area.y;

        // BTB indicator
        if self.scoring.btb > 0 {
            let btb_text = format!("B2B ×{}", self.scoring.btb);
            let color = theme::btb_color(self.scoring.btb as u32);
            let style = Style::default().fg(color).add_modifier(Modifier::BOLD);
            buf.set_string(x, y, &btb_text, style);
            y += 1;
        }

        // Clear type
        if let Some(clear_type) = self.clear_type {
            if !matches!(clear_type, ClearType::None) {
                let text = clear_type.display_name();
                let color = theme::clear_type_color(clear_type);
                let style = Style::default().fg(color).add_modifier(Modifier::BOLD);
                if y < area.y + area.height {
                    buf.set_string(x, y, text, style);
                    y += 1;
                }
            }
        }

        // Combo
        if self.scoring.combo > 0 {
            let combo_text = format!("{} COMBO", self.scoring.combo);
            let color = theme::combo_color(self.scoring.combo as u32);
            let style = Style::default().fg(color).add_modifier(Modifier::BOLD);
            if y < area.y + area.height {
                buf.set_string(x, y, &combo_text, style);
            }
        }
    }
}
