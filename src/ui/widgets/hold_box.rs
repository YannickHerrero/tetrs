use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

use crate::game::piece::{get_cells, PieceType, RotationState};
use crate::ui::theme;

/// Widget that renders the hold piece box.
pub struct HoldBoxWidget {
    pub piece: Option<PieceType>,
    pub available: bool,
}

impl Widget for HoldBoxWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 10 || area.height < 5 {
            return;
        }

        let x = area.x;
        let mut y = area.y;

        // Header
        buf.set_string(x + 1, y, "HOLD", theme::header_style());
        y += 1;
        let sep: String = theme::PANEL_H.repeat(area.width as usize);
        buf.set_string(x, y, &sep, Style::default().fg(theme::PANEL_COLOR));
        y += 1;

        if let Some(piece_type) = self.piece {
            let color = if self.available {
                piece_type.color()
            } else {
                piece_type.dim_color()
            };

            let cells = get_cells(piece_type, RotationState::R0);
            let style = Style::default().fg(color);

            let min_x = cells.iter().map(|c| c.0).min().unwrap_or(0);
            let max_y = cells.iter().map(|c| c.1).max().unwrap_or(0);

            let offset_x = match piece_type {
                PieceType::O => 1,
                PieceType::I => 0,
                _ => 1,
            };

            for &(cx, cy) in &cells {
                let sx = x + 1 + offset_x + (cx - min_x) as u16 * 2;
                let sy = y + (max_y - cy) as u16;
                if sx + 1 < buf.area.width && sy < buf.area.height {
                    buf.set_string(sx, sy, theme::BLOCK_FULL, style);
                }
            }
        }
    }
}
