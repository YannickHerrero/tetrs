use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;

use crate::game::board::{Board, Cell, BOARD_WIDTH, VISIBLE_HEIGHT};
use crate::game::ghost;
use crate::game::piece::Piece;
use crate::ui::effects::Effects;
use crate::ui::theme;

/// Widget that renders the Tetris playfield.
pub struct BoardWidget<'a> {
    pub board: &'a Board,
    pub current_piece: Option<&'a Piece>,
    pub effects: &'a Effects,
    pub show_grid: bool,
}

impl<'a> Widget for BoardWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let board_width = BOARD_WIDTH as u16 * 2; // Each cell is 2 chars wide
        let board_height = VISIBLE_HEIGHT as u16;

        // Calculate board position within the area
        if area.width < board_width + 2 || area.height < board_height + 2 {
            return; // Area too small
        }

        let shake = self.effects.shake_offset();
        let board_x = (area.x as i16 + 1 + shake.0).max(area.x as i16) as u16;
        let board_y = (area.y as i16 + 1 + shake.1).max(area.y as i16) as u16;

        // Draw border
        self.draw_border(area, buf);

        // Draw board cells
        for vis_row in 0..VISIBLE_HEIGHT {
            let board_row = vis_row;
            let screen_row = board_y + (VISIBLE_HEIGHT - 1 - vis_row) as u16;

            if screen_row >= area.y + area.height {
                continue;
            }

            for col in 0..BOARD_WIDTH {
                let screen_col = board_x + col as u16 * 2;

                if screen_col + 1 >= area.x + area.width {
                    continue;
                }

                let (text, style) = self.cell_display(col as i32, board_row as i32);

                if screen_col < buf.area.width && screen_row < buf.area.height {
                    buf.set_string(screen_col, screen_row, text, style);
                }
            }
        }
    }
}

impl<'a> BoardWidget<'a> {
    fn cell_display(&self, col: i32, row: i32) -> (&'static str, Style) {
        // Check line clear flash
        if let Some((ref flash_rows, progress)) = self.effects.line_clear_flash {
            if flash_rows.contains(&(row as usize)) {
                let color = self.effects.line_clear_color(progress);
                return (theme::BLOCK_FLASH, Style::default().fg(color));
            }
        }

        // Check active piece
        if let Some(piece) = self.current_piece {
            let cells = piece.cells();
            for &(cx, cy) in &cells {
                if cx == col && cy == row {
                    // Lock flash effect
                    let color = if !self.effects.lock_flash.is_zero() {
                        piece.piece_type.bright_color()
                    } else {
                        piece.piece_type.bright_color()
                    };
                    return (theme::BLOCK_FULL, Style::default().fg(color));
                }
            }

            // Check ghost piece
            let ghost_cells = ghost::ghost_cells(self.board, piece);
            let ghost_y = ghost::ghost_y(self.board, piece);
            if ghost_y != piece.y {
                for &(gx, gy) in &ghost_cells {
                    if gx == col && gy == row {
                        return (theme::BLOCK_GHOST, Style::default().fg(theme::GHOST_COLOR));
                    }
                }
            }
        }

        // Check board cells
        match self.board.get(col, row) {
            Cell::Empty => {
                // Grid pattern
                if self.show_grid {
                    (theme::BLOCK_DOT, Style::default().fg(theme::GRID_DOT_COLOR))
                } else {
                    (theme::BLOCK_EMPTY, Style::default())
                }
            }
            Cell::Filled(piece_type) => {
                let color = piece_type.color();
                // Danger zone tint for high rows
                let color = if row >= (VISIBLE_HEIGHT as i32 - 4) && self.effects.in_danger {
                    blend_color(color, theme::DANGER_COLOR, self.effects.danger_flash * 0.3)
                } else {
                    color
                };
                (theme::BLOCK_FULL, Style::default().fg(color))
            }
            Cell::Garbage => {
                let color = if row % 2 == 0 {
                    theme::GARBAGE_COLOR
                } else {
                    theme::GARBAGE_DARK
                };
                (theme::BLOCK_GARBAGE, Style::default().fg(color))
            }
        }
    }

    fn draw_border(&self, area: Rect, buf: &mut Buffer) {
        let w = BOARD_WIDTH as u16 * 2 + 2;
        let h = VISIBLE_HEIGHT as u16 + 2;

        let border_color = if self.effects.in_danger {
            blend_color(
                theme::BORDER_COLOR,
                theme::DANGER_COLOR,
                self.effects.danger_flash * 0.6,
            )
        } else if !self.effects.hard_drop_flash.is_zero() {
            theme::BORDER_BRIGHT
        } else {
            theme::BORDER_COLOR
        };
        let style = Style::default().fg(border_color);

        let x = area.x;
        let y = area.y;

        // Top border (lower half blocks — fill toward the playfield below)
        buf.set_string(x, y, theme::BORDER_TL, style);
        for i in 1..w - 1 {
            buf.set_string(x + i, y, theme::BORDER_H_TOP, style);
        }
        buf.set_string(x + w - 1, y, theme::BORDER_TR, style);

        // Side borders (half blocks — fill toward the playfield inside)
        for row in 1..h - 1 {
            // Gradient: brighter near top
            let t = row as f32 / (h - 2) as f32;
            let grad_color = blend_color(theme::BORDER_BRIGHT, theme::BORDER_COLOR, t);
            let grad_style = Style::default().fg(if self.effects.in_danger {
                blend_color(
                    grad_color,
                    theme::DANGER_COLOR,
                    self.effects.danger_flash * 0.4,
                )
            } else {
                grad_color
            });
            buf.set_string(x, y + row, theme::BORDER_V_LEFT, grad_style);
            buf.set_string(x + w - 1, y + row, theme::BORDER_V_RIGHT, grad_style);
        }

        // Bottom border (upper half blocks — fill toward the playfield above)
        buf.set_string(x, y + h - 1, theme::BORDER_BL, style);
        for i in 1..w - 1 {
            buf.set_string(x + i, y + h - 1, theme::BORDER_H_BOT, style);
        }
        buf.set_string(x + w - 1, y + h - 1, theme::BORDER_BR, style);
    }
}

/// Blend two colors. t=0 gives c1, t=1 gives c2.
pub fn blend_color(c1: Color, c2: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    match (c1, c2) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = (r1 as f32 * (1.0 - t) + r2 as f32 * t) as u8;
            let g = (g1 as f32 * (1.0 - t) + g2 as f32 * t) as u8;
            let b = (b1 as f32 * (1.0 - t) + b2 as f32 * t) as u8;
            Color::Rgb(r, g, b)
        }
        _ => c1,
    }
}
