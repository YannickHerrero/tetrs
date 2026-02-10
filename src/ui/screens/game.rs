use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Widget;

use crate::game::GameState;
use crate::ui::effects::Effects;
use crate::ui::layout::SingleLayout;
use crate::ui::theme;
use crate::ui::widgets::action_text::ActionTextWidget;
use crate::ui::widgets::board::BoardWidget;
use crate::ui::widgets::garbage_bar::GarbageBarWidget;
use crate::ui::widgets::hold_box::HoldBoxWidget;
use crate::ui::widgets::next_queue::NextQueueWidget;
use crate::ui::widgets::sidebar::SidebarWidget;

/// Widget that renders the full game screen (single-player layout).
pub struct GameScreen<'a> {
    pub game: &'a GameState,
    pub effects: &'a Effects,
    pub layout: &'a SingleLayout,
    pub mode_name: &'a str,
    pub mode_info: Option<&'a str>,
    pub paused: bool,
}

impl<'a> Widget for GameScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Clear background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                buf.set_string(x, y, " ", Style::default().bg(theme::BG_COLOR));
            }
        }

        // Hold box
        HoldBoxWidget {
            piece: self.game.hold.piece,
            available: !self.game.hold.used_this_turn,
        }
        .render(self.layout.hold, buf);

        // Board
        BoardWidget {
            board: &self.game.board,
            current_piece: self.game.current_piece.as_ref(),
            effects: self.effects,
            show_grid: true,
        }
        .render(self.layout.board, buf);

        // Garbage bar
        GarbageBarWidget {
            pending: self.game.garbage.pending(),
        }
        .render(self.layout.garbage_bar, buf);

        // Next queue
        NextQueueWidget {
            pieces: self.game.preview(),
        }
        .render(self.layout.next, buf);

        // Left sidebar (score)
        SidebarWidget {
            scoring: &self.game.scoring,
            stats: &self.game.stats,
            title: self.mode_name,
        }
        .render(self.layout.left_sidebar, buf);

        // Right sidebar (stats)
        self.render_right_sidebar(buf);

        // Action text
        if self.effects.action_text_opacity > 0.01 {
            ActionTextWidget {
                clear_type: self.game.last_clear_type.as_ref(),
                scoring: &self.game.scoring,
                opacity: self.effects.action_text_opacity,
            }
            .render(self.layout.action_text, buf);
        }

        // Mode-specific info (like lines remaining for sprint)
        if let Some(info) = self.mode_info {
            let info_y = self.layout.board.y + self.layout.board.height;
            let info_x = self.layout.board.x + 1;
            buf.set_string(info_x, info_y, info, theme::stat_value_style());
        }

        // Controls bar
        self.render_controls(buf);

        // Pause overlay
        if self.paused {
            self.render_pause_overlay(area, buf);
        }
    }
}

impl<'a> GameScreen<'a> {
    fn render_right_sidebar(&self, buf: &mut Buffer) {
        let area = self.layout.right_sidebar;
        if area.width < 8 || area.height < 4 {
            return;
        }

        let x = area.x;
        let mut y = area.y;

        // APM
        buf.set_string(x + 1, y, "APM", theme::stat_label_style());
        y += 1;
        buf.set_string(
            x + 1,
            y,
            &format!("{:.1}", self.game.stats.apm()),
            theme::stat_value_style(),
        );
        y += 2;

        // Max combo
        if y + 1 < area.y + area.height {
            buf.set_string(x + 1, y, "MAX CMB", theme::stat_label_style());
            y += 1;
            buf.set_string(
                x + 1,
                y,
                &self.game.stats.max_combo.to_string(),
                theme::stat_value_style(),
            );
            y += 2;
        }

        // Attack
        if y + 1 < area.y + area.height {
            buf.set_string(x + 1, y, "ATTACK", theme::stat_label_style());
            y += 1;
            buf.set_string(
                x + 1,
                y,
                &self.game.stats.attack_sent.to_string(),
                theme::stat_value_style(),
            );
        }
    }

    fn render_controls(&self, buf: &mut Buffer) {
        let area = self.layout.controls;
        if area.width < 30 || area.height < 1 {
            return;
        }

        let controls = "h/l:←→  j:↓  k/Space:drop  d/f:rot  s:180  g:hold  Esc:pause";
        let x = area.x + area.width.saturating_sub(controls.len() as u16) / 2;
        buf.set_string(x, area.y + 1, controls, theme::menu_desc_style());
    }

    fn render_pause_overlay(&self, area: Rect, buf: &mut Buffer) {
        // Semi-transparent overlay
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                buf.set_string(x, y, " ", Style::default().bg(theme::BG_COLOR));
            }
        }

        let center_x = area.x + area.width / 2;
        let center_y = area.y + area.height / 2;

        let text = "║  PAUSED  ║";
        let x = center_x.saturating_sub(text.len() as u16 / 2);
        let style = Style::default()
            .fg(theme::TEXT_BRIGHT)
            .add_modifier(Modifier::BOLD);

        buf.set_string(x, center_y - 2, "╔══════════╗", style);
        buf.set_string(x, center_y - 1, "║          ║", style);
        buf.set_string(x, center_y, text, style);
        buf.set_string(x, center_y + 1, "║          ║", style);
        buf.set_string(x, center_y + 2, "╚══════════╝", style);

        let resume = "[Esc] Resume   [R] Restart   [Q] Quit";
        let rx = center_x.saturating_sub(resume.len() as u16 / 2);
        buf.set_string(rx, center_y + 4, resume, theme::menu_desc_style());
    }
}
