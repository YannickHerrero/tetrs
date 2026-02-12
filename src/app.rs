use std::time::Duration;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

use crate::ai::difficulty::AiDifficulty;
use crate::data::high_scores::HighScoreStore;
use crate::game::{GameEvent, GameState};
use crate::input::{AppInput, InputHandler};
use crate::modes::endless::EndlessMode;
use crate::modes::sprint::SprintMode;
use crate::modes::versus::VersusMode;
use crate::modes::GameMode;
use crate::ui::effects::Effects;
use crate::ui::layout::{self, SingleLayout, VersusLayout};
use crate::ui::screens::game::GameScreen;
use crate::ui::screens::game_over::{GameOverScreen, GameResult};
use crate::ui::screens::high_scores::HighScoresScreen;
use crate::ui::screens::menu::{MenuChoice, MenuScreen};
use crate::ui::theme;

/// Top-level application state.
pub enum AppState {
    Menu,
    DifficultySelect,
    Playing,
    Paused,
    GameOver,
    HighScores,
    Quitting,
}

/// The main application.
pub struct App {
    pub state: AppState,
    pub menu: MenuScreen,
    pub game: GameState,
    pub mode: Option<Box<dyn GameMode>>,
    pub input: InputHandler,
    pub effects: Effects,
    pub high_scores: HighScoreStore,
    pub game_result: Option<GameResult>,
    pub frame: u64,

    // Versus mode specific
    pub versus_mode: Option<VersusMode>,
    pub ai_effects: Effects,
    pub last_player_attack: u32,

    // Difficulty selection
    pub difficulty_selected: usize,

    // High scores tab
    pub hs_tab: usize,
}

impl App {
    pub fn new(has_key_release: bool) -> Self {
        Self {
            state: AppState::Menu,
            menu: MenuScreen::new(),
            game: GameState::new(),
            mode: None,
            input: InputHandler::new(has_key_release),
            effects: Effects::new(),
            high_scores: HighScoreStore::load(),
            game_result: None,
            frame: 0,
            versus_mode: None,
            ai_effects: Effects::new(),
            last_player_attack: 0,
            difficulty_selected: 1, // Default to Medium
            hs_tab: 0,
        }
    }

    /// Process one frame. Returns false if the app should exit.
    pub fn update(&mut self, dt: Duration) -> bool {
        self.frame += 1;

        // Poll input
        let inputs = self.input.poll();

        // Handle inputs based on state
        for input in inputs {
            if !self.handle_input(input) {
                return false;
            }
        }

        // Game-specific updates
        match &self.state {
            AppState::Playing => {
                // DAS-driven actions
                let das_actions = self.input.tick_das(dt);
                for action in das_actions {
                    self.game.handle_action(action);
                }

                // Game tick
                let _tick_result = self.game.update(dt);

                // Process game events
                self.process_events();

                // Update effects
                self.effects.set_danger(self.game.is_danger());
                self.effects.update(dt);

                // Update versus AI
                if let Some(ref mut vs) = self.versus_mode {
                    // Feed player attack to AI as garbage
                    let current_player_attack = self.game.stats.attack_sent;
                    let player_delta =
                        current_player_attack.saturating_sub(self.last_player_attack);
                    self.last_player_attack = current_player_attack;
                    if player_delta > 0 {
                        vs.ai_game.garbage.add(player_delta);
                    }

                    vs.update_ai(dt);

                    // Feed AI attack to player as garbage
                    let ai_attack = vs.ai.check_attack(&vs.ai_game);
                    if ai_attack > 0 {
                        self.game.garbage.add(ai_attack);
                    }

                    self.ai_effects.set_danger(vs.ai_game.is_danger());
                    self.ai_effects.update(dt);
                }

                // Check mode completion
                if let Some(ref mode) = self.mode {
                    if let Some(mut result) = mode.check_complete(&self.game) {
                        // Check high scores
                        result.is_new_high_score = self.check_and_save_high_score(&result);
                        self.game_result = Some(result);
                        self.state = AppState::GameOver;
                        self.input.in_game = false;
                    }
                }

                // Also check versus mode completion
                if let Some(ref vs) = self.versus_mode {
                    let player_dead = self.game.game_over;
                    let ai_dead = vs.ai_game.game_over;
                    if player_dead || ai_dead {
                        let won = ai_dead && !player_dead;
                        let mut result = GameResult {
                            mode_name: format!("Versus AI ({})", vs.difficulty.name()),
                            primary_label: if won { "VICTORY" } else { "DEFEAT" }.to_string(),
                            primary_value: format!(
                                "ATK: {} | RCV: {}",
                                self.game.stats.attack_sent, self.game.stats.garbage_received
                            ),
                            is_new_high_score: false,
                            stats: self.game.stats.clone(),
                            won: Some(won),
                        };
                        result.is_new_high_score = self.check_and_save_high_score(&result);
                        self.game_result = Some(result);
                        self.state = AppState::GameOver;
                        self.input.in_game = false;
                    }
                }
            }
            AppState::Menu => {
                self.menu.frame = self.frame;
            }
            AppState::GameOver => {}
            _ => {}
        }

        true
    }

    /// Handle a single input event. Returns false to quit.
    fn handle_input(&mut self, input: AppInput) -> bool {
        match &self.state {
            AppState::Menu => match input {
                AppInput::MenuUp => self.menu.move_up(),
                AppInput::MenuDown => self.menu.move_down(),
                AppInput::MenuSelect => match self.menu.selected_choice() {
                    MenuChoice::Sprint => self.start_sprint(),
                    MenuChoice::Endless => self.start_endless(),
                    MenuChoice::Versus => {
                        self.state = AppState::DifficultySelect;
                    }
                    MenuChoice::HighScores => {
                        self.state = AppState::HighScores;
                    }
                    MenuChoice::Quit => return false,
                },
                AppInput::Quit | AppInput::MenuBack => return false,
                _ => {}
            },
            AppState::DifficultySelect => match input {
                AppInput::MenuUp => {
                    if self.difficulty_selected > 0 {
                        self.difficulty_selected -= 1;
                    } else {
                        self.difficulty_selected = 3;
                    }
                }
                AppInput::MenuDown => {
                    self.difficulty_selected = (self.difficulty_selected + 1) % 4;
                }
                AppInput::MenuSelect => {
                    let diff = match self.difficulty_selected {
                        0 => AiDifficulty::Easy,
                        1 => AiDifficulty::Medium,
                        2 => AiDifficulty::Hard,
                        _ => AiDifficulty::Expert,
                    };
                    self.start_versus(diff);
                }
                AppInput::MenuBack | AppInput::Quit => {
                    self.state = AppState::Menu;
                }
                _ => {}
            },
            AppState::Playing => match input {
                AppInput::Game(action) => {
                    self.game.handle_action(action);
                }
                AppInput::Pause => {
                    self.state = AppState::Paused;
                    self.input.reset_das();
                }
                AppInput::Restart => {
                    self.restart_game();
                }
                AppInput::Quit => {
                    self.state = AppState::Menu;
                    self.input.in_game = false;
                    self.input.reset_das();
                }
                _ => {}
            },
            AppState::Paused => match input {
                AppInput::Pause => {
                    self.state = AppState::Playing;
                }
                AppInput::Restart => {
                    self.restart_game();
                    self.state = AppState::Playing;
                }
                AppInput::Quit | AppInput::MenuBack => {
                    self.state = AppState::Menu;
                    self.input.in_game = false;
                    self.input.reset_das();
                }
                _ => {}
            },
            AppState::GameOver => match input {
                AppInput::Restart | AppInput::MenuSelect => {
                    self.restart_game();
                }
                AppInput::MenuBack | AppInput::Quit => {
                    self.state = AppState::Menu;
                    self.input.in_game = false;
                }
                _ => {}
            },
            AppState::HighScores => match input {
                AppInput::MenuBack | AppInput::Quit => {
                    self.state = AppState::Menu;
                }
                AppInput::MenuLeft => {
                    if self.hs_tab > 0 {
                        self.hs_tab -= 1;
                    }
                }
                AppInput::MenuRight => {
                    if self.hs_tab < 2 {
                        self.hs_tab += 1;
                    }
                }
                _ => {}
            },
            AppState::Quitting => return false,
        }
        true
    }

    fn start_sprint(&mut self) {
        self.game = GameState::new();
        self.effects.reset();
        let mode = SprintMode::new();
        self.mode = Some(Box::new(mode));
        self.versus_mode = None;
        self.game.start();
        self.state = AppState::Playing;
        self.input.in_game = true;
        self.input.reset_das();
    }

    fn start_endless(&mut self) {
        self.game = GameState::new();
        self.effects.reset();
        let mode = EndlessMode::new();
        self.mode = Some(Box::new(mode));
        self.versus_mode = None;
        self.game.start();
        self.state = AppState::Playing;
        self.input.in_game = true;
        self.input.reset_das();
    }

    fn start_versus(&mut self, difficulty: AiDifficulty) {
        self.game = GameState::new();
        self.effects.reset();
        self.ai_effects.reset();
        self.last_player_attack = 0;
        let mut vs = VersusMode::new(difficulty);
        self.game.start();
        vs.ai_game.start();
        vs.ai.reset();
        self.versus_mode = Some(vs);
        self.mode = None; // Versus handles its own completion
        self.state = AppState::Playing;
        self.input.in_game = true;
        self.input.reset_das();
    }

    fn restart_game(&mut self) {
        self.effects.reset();
        self.last_player_attack = 0;
        self.game = GameState::new();
        self.game.start();
        if let Some(ref mut vs) = self.versus_mode {
            self.ai_effects.reset();
            vs.ai_game = GameState::new();
            vs.ai_game.start();
            vs.ai.reset();
        }
        self.state = AppState::Playing;
        self.input.in_game = true;
        self.input.reset_das();
    }

    fn process_events(&mut self) {
        let events = self.game.drain_events();
        for event in events {
            match event {
                GameEvent::HardDrop { cells } => {
                    self.effects.trigger_hard_drop(cells);
                }
                GameEvent::PieceLocked => {
                    self.effects.trigger_lock();
                }
                GameEvent::LinesClear(rows) => {
                    self.effects.trigger_line_clear(rows);
                }
                GameEvent::PerfectClear => {
                    self.effects.trigger_pc();
                }
                GameEvent::TSpin(_) | GameEvent::Combo(_) | GameEvent::BackToBack(_) => {
                    self.effects.trigger_action_text();
                }
                _ => {}
            }
        }
    }

    fn check_and_save_high_score(&mut self, result: &GameResult) -> bool {
        if result.won == Some(false) {
            // Don't save losses that aren't versus
        }

        let stats = &result.stats;
        if result.mode_name.contains("Sprint") {
            self.high_scores.add_sprint(
                stats.time.as_millis() as u64,
                stats.lines_cleared,
                stats.pieces_placed,
            )
        } else if result.mode_name.contains("Endless") {
            self.high_scores
                .add_endless(stats.score, stats.level, stats.lines_cleared)
        } else if result.mode_name.contains("Versus") {
            let won = result.won.unwrap_or(false);
            let diff_name = if let Some(ref vs) = self.versus_mode {
                vs.difficulty.name().to_string()
            } else {
                "Unknown".to_string()
            };
            self.high_scores.add_versus(
                won,
                &diff_name,
                stats.time.as_millis() as u64,
                stats.attack_sent,
            )
        } else {
            false
        }
    }

    /// Render the current frame.
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        match &self.state {
            AppState::Menu => {
                (&self.menu).render(area, buf);
            }
            AppState::DifficultySelect => {
                self.render_difficulty_select(area, buf);
            }
            AppState::Playing | AppState::Paused => {
                if self.versus_mode.is_some() {
                    self.render_versus(area, buf);
                } else {
                    self.render_single(area, buf);
                }
            }
            AppState::GameOver => {
                if let Some(ref result) = self.game_result {
                    GameOverScreen {
                        result,
                        frame: self.frame,
                    }
                    .render(area, buf);
                }
            }
            AppState::HighScores => {
                HighScoresScreen {
                    store: &self.high_scores,
                    selected_tab: self.hs_tab,
                }
                .render(area, buf);
            }
            AppState::Quitting => {}
        }
    }

    fn render_single(&self, area: Rect, buf: &mut Buffer) {
        if !layout::check_size_single(area) {
            self.render_size_error(area, buf, layout::MIN_WIDTH, layout::MIN_HEIGHT);
            return;
        }

        let layout = SingleLayout::new(area);
        let mode_name = self.mode.as_ref().map(|m| m.name()).unwrap_or("GAME");
        let mode_info = self.mode.as_ref().and_then(|m| m.info_text(&self.game));

        GameScreen {
            game: &self.game,
            effects: &self.effects,
            layout: &layout,
            mode_name,
            mode_info: mode_info.as_deref(),
            paused: matches!(self.state, AppState::Paused),
        }
        .render(area, buf);
    }

    fn render_versus(&self, area: Rect, buf: &mut Buffer) {
        // For versus, render two boards side by side
        // If terminal too small, show just the player board
        if !layout::check_size_single(area) {
            self.render_size_error(area, buf, layout::MIN_WIDTH, layout::MIN_HEIGHT);
            return;
        }

        // Clear background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                buf.set_string(x, y, " ", Style::default().bg(theme::BG_COLOR));
            }
        }

        if layout::check_size_versus(area) {
            let vs_layout = VersusLayout::new(area);

            // Player board (left)
            let mode_info = self.versus_mode.as_ref().map(|_| {
                format!(
                    "ATK:{} RCV:{}",
                    self.game.stats.attack_sent, self.game.stats.garbage_received
                )
            });

            GameScreen {
                game: &self.game,
                effects: &self.effects,
                layout: &vs_layout.player,
                mode_name: "PLAYER",
                mode_info: mode_info.as_deref(),
                paused: matches!(self.state, AppState::Paused),
            }
            .render(area, buf);

            // AI board (right)
            if let Some(ref vs) = self.versus_mode {
                let ai_info = format!(
                    "ATK:{} RCV:{}",
                    vs.ai_game.stats.attack_sent, vs.ai_game.stats.garbage_received
                );

                GameScreen {
                    game: &vs.ai_game,
                    effects: &self.ai_effects,
                    layout: &vs_layout.ai,
                    mode_name: &format!("AI ({})", vs.difficulty.name()),
                    mode_info: Some(&ai_info),
                    paused: matches!(self.state, AppState::Paused),
                }
                .render(area, buf);

                // VS text in center
                let center = vs_layout.center;
                buf.set_string(
                    center.x,
                    center.y + 1,
                    " VS ",
                    Style::default()
                        .fg(ratatui::style::Color::Rgb(255, 100, 100))
                        .add_modifier(ratatui::style::Modifier::BOLD),
                );
            }
        } else {
            // Fallback: just show player board
            let layout = SingleLayout::new(area);
            let mode_info = self.versus_mode.as_ref().map(|_| {
                format!(
                    "ATK:{} RCV:{}",
                    self.game.stats.attack_sent, self.game.stats.garbage_received
                )
            });

            GameScreen {
                game: &self.game,
                effects: &self.effects,
                layout: &layout,
                mode_name: "VERSUS",
                mode_info: mode_info.as_deref(),
                paused: matches!(self.state, AppState::Paused),
            }
            .render(area, buf);
        }
    }

    fn render_difficulty_select(&self, area: Rect, buf: &mut Buffer) {
        // Clear background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                buf.set_string(x, y, " ", Style::default().bg(theme::BG_COLOR));
            }
        }

        let center_x = area.x + area.width / 2;
        let mut y = area.y + area.height / 4;

        let title = "SELECT AI DIFFICULTY";
        let tx = center_x.saturating_sub(title.len() as u16 / 2);
        buf.set_string(tx, y, title, theme::title_style());
        y += 3;

        let difficulties = [
            AiDifficulty::Easy,
            AiDifficulty::Medium,
            AiDifficulty::Hard,
            AiDifficulty::Expert,
        ];

        for (i, diff) in difficulties.iter().enumerate() {
            let is_selected = i == self.difficulty_selected;
            let cursor = if is_selected { " ▸ " } else { "   " };

            let label_style = if is_selected {
                theme::menu_selected_style()
            } else {
                theme::menu_item_style()
            };

            let line = format!("{}{}", cursor, diff.name());
            let x = center_x.saturating_sub(15);
            buf.set_string(x, y, &line, label_style);

            if is_selected {
                y += 1;
                let desc = diff.description();
                let dx = center_x.saturating_sub(desc.len() as u16 / 2);
                buf.set_string(dx, y, desc, theme::menu_desc_style());
            }
            y += 2;
        }

        let controls = "j/k: navigate  Enter: select  Esc: back";
        let cx = center_x.saturating_sub(controls.len() as u16 / 2);
        let cy = area.y + area.height - 2;
        buf.set_string(cx, cy, controls, theme::menu_desc_style());
    }

    fn render_size_error(&self, area: Rect, buf: &mut Buffer, min_w: u16, min_h: u16) {
        let msg = format!(
            "Terminal too small: {}x{} (need {}x{})",
            area.width, area.height, min_w, min_h
        );
        let x = area.x + area.width.saturating_sub(msg.len() as u16) / 2;
        let y = area.y + area.height / 2;
        buf.set_string(x, y, &msg, theme::danger_style());
    }
}
