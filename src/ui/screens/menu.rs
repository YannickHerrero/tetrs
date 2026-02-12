use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

use crate::ui::theme;

/// Menu item definition.
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: &'static str,
    pub description: &'static str,
    pub id: MenuChoice,
}

/// Menu selections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuChoice {
    Sprint,
    Endless,
    Versus,
    HighScores,
    Quit,
}

/// The main menu screen widget.
pub struct MenuScreen {
    pub selected: usize,
    pub items: Vec<MenuItem>,
    pub frame: u64,
}

impl MenuScreen {
    pub fn new() -> Self {
        Self {
            selected: 0,
            items: vec![
                MenuItem {
                    label: "40 Lines Sprint",
                    description: "Clear 40 lines as fast as possible",
                    id: MenuChoice::Sprint,
                },
                MenuItem {
                    label: "Endless Marathon",
                    description: "Play forever, maximize your score",
                    id: MenuChoice::Endless,
                },
                MenuItem {
                    label: "Versus AI",
                    description: "Battle against a computer opponent",
                    id: MenuChoice::Versus,
                },
                MenuItem {
                    label: "High Scores",
                    description: "View your best performances",
                    id: MenuChoice::HighScores,
                },
                MenuItem {
                    label: "Quit",
                    description: "Exit the game",
                    id: MenuChoice::Quit,
                },
            ],
            frame: 0,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.items.len() - 1;
        }
    }

    pub fn move_down(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn selected_choice(&self) -> MenuChoice {
        self.items[self.selected].id
    }
}

impl Widget for &MenuScreen {
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
        let title_lines = TITLE_ART;
        for (i, line) in title_lines.iter().enumerate() {
            let hue = ((self.frame as f32 * 2.0 + i as f32 * 20.0) % 360.0) / 360.0;
            let color = hsl_to_rgb(hue, 0.7, 0.65);
            let style = Style::default().fg(color).add_modifier(Modifier::BOLD);
            let x = center_x.saturating_sub(line.width() as u16 / 2);
            if y < area.y + area.height {
                buf.set_string(x, y, line, style);
            }
            y += 1;
        }
        y += 2;

        // Menu items
        for (i, item) in self.items.iter().enumerate() {
            if y + 1 >= area.y + area.height {
                break;
            }

            let is_selected = i == self.selected;

            let label_style = if is_selected {
                theme::menu_selected_style()
            } else {
                theme::menu_item_style()
            };

            // Center based on the label only so cursor prefix doesn't shift text
            let label_w = item.label.width() as u16;
            let label_x = center_x.saturating_sub(label_w / 2);
            let cursor_x = label_x.saturating_sub(3);

            if is_selected {
                buf.set_string(cursor_x, y, " \u{25b8} ", label_style);
            }
            buf.set_string(label_x, y, item.label, label_style);

            // Description (only for selected)
            if is_selected {
                y += 1;
                let desc_x = center_x.saturating_sub(item.description.width() as u16 / 2);
                buf.set_string(desc_x, y, item.description, theme::menu_desc_style());
            }
            y += 1;
            if !is_selected {
                // Add spacing between non-selected items
            }
            y += 1;
        }

        // Controls help at bottom
        let controls = "j/k: navigate  Enter/Space: select  q: quit";
        let ctrl_x = center_x.saturating_sub(controls.width() as u16 / 2);
        let ctrl_y = area.y + area.height - 2;
        if ctrl_y > y {
            buf.set_string(ctrl_x, ctrl_y, controls, theme::menu_desc_style());
        }
    }
}

/// ASCII art title.
const TITLE_ART: &[&str] = &[
    "████████╗███████╗████████╗██████╗ ███████╗",
    "╚══██╔══╝██╔════╝╚══██╔══╝██╔══██╗██╔════╝",
    "   ██║   █████╗     ██║   ██████╔╝███████╗",
    "   ██║   ██╔══╝     ██║   ██╔══██╗╚════██║",
    "   ██║   ███████╗   ██║   ██║  ██║███████║",
    "   ╚═╝   ╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝",
];

/// Convert HSL to RGB color.
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match (h * 6.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Color::Rgb(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
