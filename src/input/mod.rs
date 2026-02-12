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
    /// Whether the terminal supports key release events (Kitty keyboard protocol).
    /// When false, DAS is disabled and each press/repeat is treated as a single move.
    pub has_key_release: bool,
}

impl InputHandler {
    pub fn new(has_key_release: bool) -> Self {
        Self {
            keybinds: KeybindMap::new(),
            das: DasHandler::new(),
            in_game: false,
            has_key_release,
        }
    }

    /// Poll for input events. Returns a list of immediate actions.
    /// Should be called once per frame.
    pub fn poll(&mut self) -> Vec<AppInput> {
        let mut actions = Vec::new();

        // Poll all available events (non-blocking, 1ms timeout)
        while let Ok(true) = event::poll(Duration::from_millis(1)) {
            if let Ok(Event::Key(key_event)) = event::read() {
                self.process_key(key_event, &mut actions);
            }
        }

        actions
    }

    /// Process DAS ticks. Returns additional game actions from auto-repeat.
    /// Should be called once per frame after poll().
    pub fn tick_das(&mut self, dt: Duration) -> Vec<GameAction> {
        if !self.in_game || !self.has_key_release {
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

    fn process_key(&mut self, key_event: KeyEvent, actions: &mut Vec<AppInput>) {
        let code = key_event.code;

        if self.in_game {
            self.process_game_key(code, key_event.kind, actions);
        } else if let Some(a) = self.process_menu_key(code, key_event.kind) {
            actions.push(a);
        }
    }

    fn process_game_key(&mut self, code: KeyCode, kind: KeyEventKind, actions: &mut Vec<AppInput>) {
        if self.has_key_release {
            self.process_game_key_with_release(code, kind, actions);
        } else {
            self.process_game_key_no_release(code, kind, actions);
        }
    }

    /// Input handling for terminals that support key release events.
    /// Uses DAS for auto-repeat with proper press/release tracking.
    fn process_game_key_with_release(
        &mut self,
        code: KeyCode,
        kind: KeyEventKind,
        actions: &mut Vec<AppInput>,
    ) {
        match kind {
            KeyEventKind::Press => {
                let Some(action) = self.keybinds.resolve_game(code) else {
                    return;
                };
                match action {
                    Action::MoveLeft => {
                        // Cancel opposite direction
                        self.das.right.release();
                        self.das.left.press();
                        actions.push(AppInput::Game(GameAction::MoveLeft));
                    }
                    Action::MoveRight => {
                        self.das.left.release();
                        self.das.right.press();
                        actions.push(AppInput::Game(GameAction::MoveRight));
                    }
                    Action::SoftDrop => {
                        self.das.soft_drop.press();
                        actions.push(AppInput::Game(GameAction::SoftDrop));
                    }
                    Action::HardDrop => actions.push(AppInput::Game(GameAction::HardDrop)),
                    Action::RotateCW => actions.push(AppInput::Game(GameAction::RotateCW)),
                    Action::RotateCCW => actions.push(AppInput::Game(GameAction::RotateCCW)),
                    Action::Rotate180 => actions.push(AppInput::Game(GameAction::Rotate180)),
                    Action::Hold => actions.push(AppInput::Game(GameAction::Hold)),
                    Action::Pause => actions.push(AppInput::Pause),
                    Action::Quit => actions.push(AppInput::Quit),
                    Action::Restart => actions.push(AppInput::Restart),
                    _ => {}
                }
            }
            KeyEventKind::Release => {
                let action = self.keybinds.resolve_game(code);
                match action {
                    Some(Action::MoveLeft) => {
                        self.das.left.release();
                    }
                    Some(Action::MoveRight) => {
                        self.das.right.release();
                    }
                    Some(Action::SoftDrop) => {
                        self.das.soft_drop.release();
                        actions.push(AppInput::Game(GameAction::SoftDropRelease));
                    }
                    _ => {}
                }
            }
            // Repeat events are handled by DAS, ignore them
            _ => {}
        }
    }

    /// Input handling for terminals without key release events.
    /// Each Press/Repeat generates exactly one action; DAS is not used.
    /// For SoftDrop, we emit both SoftDrop and SoftDropRelease so the
    /// persistent soft_dropping flag doesn't stay on between frames.
    fn process_game_key_no_release(
        &mut self,
        code: KeyCode,
        kind: KeyEventKind,
        actions: &mut Vec<AppInput>,
    ) {
        match kind {
            KeyEventKind::Press | KeyEventKind::Repeat => {
                let Some(action) = self.keybinds.resolve_game(code) else {
                    return;
                };
                match action {
                    Action::MoveLeft => actions.push(AppInput::Game(GameAction::MoveLeft)),
                    Action::MoveRight => actions.push(AppInput::Game(GameAction::MoveRight)),
                    Action::SoftDrop => {
                        // Move down one cell, then immediately release so gravity
                        // doesn't stay in soft-drop mode forever.
                        actions.push(AppInput::Game(GameAction::SoftDrop));
                        actions.push(AppInput::Game(GameAction::SoftDropRelease));
                    }
                    Action::HardDrop => {
                        // Only on initial press, not repeats, to avoid accidental hard drops
                        if kind == KeyEventKind::Press {
                            actions.push(AppInput::Game(GameAction::HardDrop));
                        }
                    }
                    Action::RotateCW => actions.push(AppInput::Game(GameAction::RotateCW)),
                    Action::RotateCCW => actions.push(AppInput::Game(GameAction::RotateCCW)),
                    Action::Rotate180 => actions.push(AppInput::Game(GameAction::Rotate180)),
                    Action::Hold => {
                        if kind == KeyEventKind::Press {
                            actions.push(AppInput::Game(GameAction::Hold));
                        }
                    }
                    Action::Pause => actions.push(AppInput::Pause),
                    Action::Quit => actions.push(AppInput::Quit),
                    Action::Restart => actions.push(AppInput::Restart),
                    _ => {}
                }
            }
            // No release events expected; ignore anything else
            _ => {}
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
