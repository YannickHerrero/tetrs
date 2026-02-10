use ratatui::layout::Rect;

use crate::game::board::{BOARD_WIDTH, VISIBLE_HEIGHT};

/// Minimum terminal size for single player.
pub const MIN_WIDTH: u16 = 56;
pub const MIN_HEIGHT: u16 = 26;

/// Minimum terminal size for versus mode.
pub const VS_MIN_WIDTH: u16 = 106;
pub const VS_MIN_HEIGHT: u16 = 26;

/// Board dimensions in terminal cells.
pub const BOARD_CELL_W: u16 = BOARD_WIDTH as u16 * 2; // 20
pub const BOARD_CELL_H: u16 = VISIBLE_HEIGHT as u16; // 20
pub const BOARD_TOTAL_W: u16 = BOARD_CELL_W + 2; // +border
pub const BOARD_TOTAL_H: u16 = BOARD_CELL_H + 2; // +border

/// Panel width for sidebars.
pub const PANEL_W: u16 = 12;

/// Layout positions for single-player mode.
#[derive(Debug, Clone)]
pub struct SingleLayout {
    pub hold: Rect,
    pub board: Rect,
    pub next: Rect,
    pub left_sidebar: Rect,
    pub right_sidebar: Rect,
    pub garbage_bar: Rect,
    pub action_text: Rect,
    pub controls: Rect,
}

/// Layout positions for versus mode.
#[derive(Debug, Clone)]
pub struct VersusLayout {
    pub player: SingleLayout,
    pub ai: SingleLayout,
    pub center: Rect,
}

impl SingleLayout {
    /// Calculate layout centered in the given area.
    pub fn new(area: Rect) -> Self {
        let total_w = PANEL_W + 1 + BOARD_TOTAL_W + 1 + PANEL_W;
        let total_h = BOARD_TOTAL_H + 2; // +controls row

        let start_x = area.x + area.width.saturating_sub(total_w) / 2;
        let start_y = area.y + area.height.saturating_sub(total_h) / 2;

        let left_x = start_x;
        let board_x = left_x + PANEL_W + 1;
        let right_x = board_x + BOARD_TOTAL_W + 1;

        SingleLayout {
            hold: Rect::new(left_x, start_y, PANEL_W, 5),
            left_sidebar: Rect::new(
                left_x,
                start_y + 6,
                PANEL_W,
                BOARD_TOTAL_H.saturating_sub(6),
            ),
            board: Rect::new(board_x, start_y, BOARD_TOTAL_W, BOARD_TOTAL_H),
            garbage_bar: Rect::new(board_x.saturating_sub(1), start_y + 1, 1, BOARD_CELL_H),
            next: Rect::new(right_x, start_y, PANEL_W, 12),
            right_sidebar: Rect::new(
                right_x,
                start_y + 12,
                PANEL_W,
                BOARD_TOTAL_H.saturating_sub(12),
            ),
            action_text: Rect::new(board_x + 2, start_y + BOARD_TOTAL_H / 2, BOARD_CELL_W, 4),
            controls: Rect::new(start_x, start_y + BOARD_TOTAL_H, total_w, 2),
        }
    }
}

impl VersusLayout {
    /// Calculate versus layout centered in the given area.
    pub fn new(area: Rect) -> Self {
        let single_w = PANEL_W + 1 + BOARD_TOTAL_W + 1 + PANEL_W;
        let gap = 4;
        let total_w = single_w * 2 + gap;

        let start_x = area.x + area.width.saturating_sub(total_w) / 2;
        let start_y = area.y + area.height.saturating_sub(BOARD_TOTAL_H + 2) / 2;

        let player_area = Rect::new(start_x, start_y, single_w, area.height);
        let ai_area = Rect::new(start_x + single_w + gap, start_y, single_w, area.height);
        let center_area = Rect::new(start_x + single_w, start_y + BOARD_TOTAL_H / 2 - 2, gap, 5);

        VersusLayout {
            player: SingleLayout::new(player_area),
            ai: SingleLayout::new(ai_area),
            center: center_area,
        }
    }
}

/// Check if the terminal is big enough for single player.
pub fn check_size_single(area: Rect) -> bool {
    area.width >= MIN_WIDTH && area.height >= MIN_HEIGHT
}

/// Check if the terminal is big enough for versus.
pub fn check_size_versus(area: Rect) -> bool {
    area.width >= VS_MIN_WIDTH && area.height >= VS_MIN_HEIGHT
}
