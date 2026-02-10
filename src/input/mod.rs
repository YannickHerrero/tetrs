pub mod das;
pub mod keybinds;

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use self::das::DasHandler;
use self::keybinds::{Action, KeybindMap};
use crate::game::GameAction;

/// Input event for the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppInput {
    Game(GameAction),
    Pause,
    Quit,
    Restart,
    MenuUp,
    MenuDown,
    MenuLeft,
    MenuRight,
    MenuSelect,
    MenuBack,
    None,
}

/// Input handler: polls crossterm events, manages DAS, maps keys to actions.
pub struct InputHandler {
    pub keybinds: KeybindMap,
    pub das: DasHandler,
    /// Whether we're in game mode (DAS active) or menu mode.
    pub in_game: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            keybinds: KeybindMap::new(),
            das: DasHandler::new(),
            in_game: false,
        }
    }

    /// Poll for input events. Returns a list of immediate actions.
    /// Should be called once per frame.
    pub fn poll(&mut self) -> Vec<AppInput> {
        let mut actions = Vec::new();

        // Poll all available events (non-blocking, 1ms timeout)
        while let Ok(true) = event::poll(Duration::from_millis(1)) {
            if let Ok(Event::Key(key_event)) = event::read() {
                if let Some(action) = self.process_key(key_event) {
                    actions.push(action);
                }
            }
        }

        actions
    }

    /// Process DAS ticks. Returns additional game actions from auto-repeat.
    /// Should be called once per frame after poll().
    pub fn tick_das(&mut self, dt: Duration) -> Vec<GameAction> {
        if !self.in_game {
            return Vec::new();
        }

        let (left, right, _sd) = self.das.tick(dt);
        let mut actions = Vec::new();

        for _ in 0..left {
            actions.push(GameAction::MoveLeft);
        }
        for _ in 0..right {
            actions.push(GameAction::MoveRight);
        }

        actions
    }

    fn process_key(&mut self, key_event: KeyEvent) -> Option<AppInput> {
        let code = key_event.code;

        if self.in_game {
            self.process_game_key(code, key_event.kind)
        } else {
            self.process_menu_key(code, key_event.kind)
        }
    }

    fn process_game_key(&mut self, code: KeyCode, kind: KeyEventKind) -> Option<AppInput> {
        match kind {
            KeyEventKind::Press => {
                let action = self.keybinds.resolve_game(code)?;
                match action {
                    Action::MoveLeft => {
                        // Cancel opposite direction
                        self.das.right.release();
                        self.das.left.press();
                        Some(AppInput::Game(GameAction::MoveLeft))
                    }
                    Action::MoveRight => {
                        self.das.left.release();
                        self.das.right.press();
                        Some(AppInput::Game(GameAction::MoveRight))
                    }
                    Action::SoftDrop => {
                        self.das.soft_drop.press();
                        Some(AppInput::Game(GameAction::SoftDrop))
                    }
                    Action::HardDrop => Some(AppInput::Game(GameAction::HardDrop)),
                    Action::RotateCW => Some(AppInput::Game(GameAction::RotateCW)),
                    Action::RotateCCW => Some(AppInput::Game(GameAction::RotateCCW)),
                    Action::Rotate180 => Some(AppInput::Game(GameAction::Rotate180)),
                    Action::Hold => Some(AppInput::Game(GameAction::Hold)),
                    Action::Pause => Some(AppInput::Pause),
                    Action::Quit => Some(AppInput::Quit),
                    Action::Restart => Some(AppInput::Restart),
                    _ => None,
                }
            }
            KeyEventKind::Release => {
                let action = self.keybinds.resolve_game(code);
                match action {
                    Some(Action::MoveLeft) => {
                        self.das.left.release();
                        // If right is still physically held, resume it
                        None
                    }
                    Some(Action::MoveRight) => {
                        self.das.right.release();
                        None
                    }
                    Some(Action::SoftDrop) => {
                        self.das.soft_drop.release();
                        Some(AppInput::Game(GameAction::SoftDropRelease))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn process_menu_key(&mut self, code: KeyCode, kind: KeyEventKind) -> Option<AppInput> {
        if kind != KeyEventKind::Press {
            return None;
        }

        let action = self.keybinds.resolve_menu(code)?;
        match action {
            Action::MenuUp => Some(AppInput::MenuUp),
            Action::MenuDown => Some(AppInput::MenuDown),
            Action::MenuLeft => Some(AppInput::MenuLeft),
            Action::MenuRight => Some(AppInput::MenuRight),
            Action::MenuSelect => Some(AppInput::MenuSelect),
            Action::MenuBack => Some(AppInput::MenuBack),
            Action::Quit => Some(AppInput::Quit),
            _ => None,
        }
    }

    /// Reset DAS state (e.g., on pause/resume).
    pub fn reset_das(&mut self) {
        self.das.reset();
    }
}
