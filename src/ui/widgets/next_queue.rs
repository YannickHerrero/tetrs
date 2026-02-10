use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

use crate::game::piece::{get_cells, PieceType, RotationState};
use crate::ui::theme;

/// Widget that renders the next piece preview queue.
pub struct NextQueueWidget {
    pub pieces: Vec<PieceType>,
}

impl Widget for NextQueueWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 10 || area.height < 3 {
            return;
        }

        let x = area.x;
        let mut y = area.y;

        // Header
        buf.set_string(x + 1, y, "NEXT", theme::header_style());
        y += 1;
        let sep: String = theme::PANEL_H.repeat(area.width as usize);
        buf.set_string(x, y, &sep, Style::default().fg(theme::PANEL_COLOR));
        y += 1;

        for (i, &piece_type) in self.pieces.iter().enumerate() {
            if y + 3 > area.y + area.height {
                break;
            }

            let color = if i == 0 {
                piece_type.bright_color()
            } else {
                piece_type.color()
            };

            let cells = get_cells(piece_type, RotationState::R0);
            self.draw_mini_piece(buf, x + 1, y, &cells, color, piece_type);
            y += 3;
        }
    }
}

impl NextQueueWidget {
    fn draw_mini_piece(
        &self,
        buf: &mut Buffer,
        x: u16,
        y: u16,
        cells: &[(i32, i32); 4],
        color: ratatui::style::Color,
        piece_type: PieceType,
    ) {
        let style = Style::default().fg(color);

        // Find bounds to center the piece
        let min_x = cells.iter().map(|c| c.0).min().unwrap_or(0);
        let max_y = cells.iter().map(|c| c.1).max().unwrap_or(0);

        // Offset for centering (I and O pieces get special treatment)
        let offset_x = match piece_type {
            PieceType::O => 1,
            PieceType::I => 0,
            _ => 1,
        };

        for &(cx, cy) in cells {
            let sx = x + offset_x + (cx - min_x) as u16 * 2;
            let sy = y + (max_y - cy) as u16;
            if sx + 1 < buf.area.width && sy < buf.area.height {
                buf.set_string(sx, sy, theme::BLOCK_FULL, style);
            }
        }
    }
}
