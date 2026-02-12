use ratatui::style::{Color, Modifier, Style};

use crate::game::clear::ClearType;

// Block characters
pub const BLOCK_FULL: &str = "██";
pub const BLOCK_GHOST: &str = "░░";
pub const BLOCK_FLASH: &str = "▓▓";
pub const BLOCK_EMPTY: &str = "  ";
pub const BLOCK_DOT: &str = "· ";
pub const BLOCK_GARBAGE: &str = "▒▒";

// Border characters (half-block elements for seamless piece-to-wall contact)
pub const BORDER_TL: &str = "▗"; // Lower-right quadrant (connects top bar + left wall)
pub const BORDER_TR: &str = "▖"; // Lower-left quadrant (connects top bar + right wall)
pub const BORDER_BL: &str = "▝"; // Upper-right quadrant (connects bottom bar + left wall)
pub const BORDER_BR: &str = "▘"; // Upper-left quadrant (connects bottom bar + right wall)
pub const BORDER_H_TOP: &str = "▄"; // Lower half block (top border, fills toward playfield)
pub const BORDER_H_BOT: &str = "▀"; // Upper half block (bottom border, fills toward playfield)
pub const BORDER_V_LEFT: &str = "▐"; // Right half block (left wall, fills toward playfield)
pub const BORDER_V_RIGHT: &str = "▌"; // Left half block (right wall, fills toward playfield)

// Panel border characters
pub const PANEL_TL: &str = "┌";
pub const PANEL_TR: &str = "┐";
pub const PANEL_BL: &str = "└";
pub const PANEL_BR: &str = "┘";
pub const PANEL_H: &str = "─";
pub const PANEL_V: &str = "│";

// Colors
pub const BG_COLOR: Color = Color::Rgb(15, 15, 20);
pub const BORDER_COLOR: Color = Color::Rgb(60, 65, 80);
pub const BORDER_BRIGHT: Color = Color::Rgb(100, 110, 140);
pub const PANEL_COLOR: Color = Color::Rgb(45, 50, 65);
pub const TEXT_COLOR: Color = Color::Rgb(200, 205, 215);
pub const TEXT_DIM: Color = Color::Rgb(100, 105, 120);
pub const TEXT_BRIGHT: Color = Color::Rgb(240, 245, 255);
pub const GHOST_COLOR: Color = Color::Rgb(60, 62, 70);
pub const GARBAGE_COLOR: Color = Color::Rgb(100, 100, 110);
pub const GARBAGE_DARK: Color = Color::Rgb(65, 65, 75);
pub const DANGER_COLOR: Color = Color::Rgb(180, 40, 40);
pub const GRID_DOT_COLOR: Color = Color::Rgb(30, 32, 40);

// Garbage bar colors
pub const GARBAGE_BAR_COLOR: Color = Color::Rgb(200, 50, 50);
pub const GARBAGE_BAR_BG: Color = Color::Rgb(35, 35, 45);

// Level colors (progression)
pub fn level_color(level: u32) -> Color {
    match level {
        0..=2 => Color::Rgb(80, 200, 120),    // Green
        3..=5 => Color::Rgb(200, 200, 60),    // Yellow
        6..=8 => Color::Rgb(230, 150, 50),    // Orange
        9..=12 => Color::Rgb(220, 60, 60),    // Red
        13..=16 => Color::Rgb(180, 60, 220),  // Purple
        17..=20 => Color::Rgb(220, 100, 255), // Bright purple
        _ => Color::Rgb(255, 180, 255),       // Pink
    }
}

// Clear type text colors
pub fn clear_type_color(clear_type: &ClearType) -> Color {
    match clear_type {
        ClearType::None => TEXT_DIM,
        ClearType::Single => Color::Rgb(180, 185, 200),
        ClearType::Double => Color::Rgb(120, 200, 255),
        ClearType::Triple => Color::Rgb(180, 100, 255),
        ClearType::Quad => Color::Rgb(255, 215, 60),
        ClearType::TSpin | ClearType::MiniTSpin => Color::Rgb(200, 80, 255),
        ClearType::TSpinSingle | ClearType::MiniTSpinSingle => Color::Rgb(220, 100, 255),
        ClearType::TSpinDouble | ClearType::MiniTSpinDouble => Color::Rgb(255, 140, 255),
        ClearType::TSpinTriple => Color::Rgb(255, 180, 255),
        ClearType::AllSpin(_) => Color::Rgb(100, 255, 200),
    }
}

// Combo color (escalating intensity)
pub fn combo_color(combo: u32) -> Color {
    match combo {
        0..=2 => TEXT_COLOR,
        3..=5 => Color::Rgb(255, 255, 100), // Yellow
        6..=9 => Color::Rgb(255, 180, 50),  // Orange
        10..=14 => Color::Rgb(255, 80, 80), // Red
        _ => Color::Rgb(255, 100, 255),     // Purple
    }
}

// BTB color
pub fn btb_color(btb: u32) -> Color {
    match btb {
        0..=2 => Color::Rgb(255, 200, 60), // Gold
        3..=5 => Color::Rgb(255, 160, 40), // Bright gold
        _ => Color::Rgb(255, 120, 200),    // Pink
    }
}

// Styles
pub fn title_style() -> Style {
    Style::default()
        .fg(Color::Rgb(100, 180, 255))
        .add_modifier(Modifier::BOLD)
}

pub fn menu_item_style() -> Style {
    Style::default().fg(TEXT_COLOR)
}

pub fn menu_selected_style() -> Style {
    Style::default()
        .fg(Color::Rgb(100, 220, 255))
        .add_modifier(Modifier::BOLD)
}

pub fn menu_desc_style() -> Style {
    Style::default().fg(TEXT_DIM)
}

pub fn header_style() -> Style {
    Style::default()
        .fg(TEXT_BRIGHT)
        .add_modifier(Modifier::BOLD)
}

pub fn stat_label_style() -> Style {
    Style::default().fg(TEXT_DIM)
}

pub fn stat_value_style() -> Style {
    Style::default()
        .fg(TEXT_BRIGHT)
        .add_modifier(Modifier::BOLD)
}

pub fn danger_style() -> Style {
    Style::default()
        .fg(DANGER_COLOR)
        .add_modifier(Modifier::BOLD)
}

pub fn game_over_style() -> Style {
    Style::default()
        .fg(Color::Rgb(255, 60, 60))
        .add_modifier(Modifier::BOLD)
}
