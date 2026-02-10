use crossterm::event::KeyCode;

/// All bindable actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    RotateCW,
    RotateCCW,
    Rotate180,
    Hold,
    Pause,
    Quit,
    Restart,
    MenuUp,
    MenuDown,
    MenuLeft,
    MenuRight,
    MenuSelect,
    MenuBack,
}

/// A key binding entry.
#[derive(Debug, Clone)]
pub struct Keybind {
    pub action: Action,
    pub keys: Vec<KeyCode>,
}

/// Default Vim-style keybinds.
pub fn default_keybinds() -> Vec<Keybind> {
    vec![
        Keybind {
            action: Action::MoveLeft,
            keys: vec![KeyCode::Char('h'), KeyCode::Left],
        },
        Keybind {
            action: Action::MoveRight,
            keys: vec![KeyCode::Char('l'), KeyCode::Right],
        },
        Keybind {
            action: Action::SoftDrop,
            keys: vec![KeyCode::Char('j'), KeyCode::Down],
        },
        Keybind {
            action: Action::HardDrop,
            keys: vec![KeyCode::Char('k'), KeyCode::Up, KeyCode::Char(' ')],
        },
        Keybind {
            action: Action::RotateCW,
            keys: vec![KeyCode::Char('f'), KeyCode::Char('x')],
        },
        Keybind {
            action: Action::RotateCCW,
            keys: vec![KeyCode::Char('d'), KeyCode::Char('z')],
        },
        Keybind {
            action: Action::Rotate180,
            keys: vec![KeyCode::Char('s'), KeyCode::Char('a')],
        },
        Keybind {
            action: Action::Hold,
            keys: vec![KeyCode::Char('g'), KeyCode::Char('c')],
        },
        Keybind {
            action: Action::Pause,
            keys: vec![KeyCode::Esc, KeyCode::Char('p')],
        },
        Keybind {
            action: Action::Quit,
            keys: vec![KeyCode::Char('q')],
        },
        Keybind {
            action: Action::Restart,
            keys: vec![KeyCode::Char('r')],
        },
        Keybind {
            action: Action::MenuUp,
            keys: vec![KeyCode::Char('k'), KeyCode::Up],
        },
        Keybind {
            action: Action::MenuDown,
            keys: vec![KeyCode::Char('j'), KeyCode::Down],
        },
        Keybind {
            action: Action::MenuLeft,
            keys: vec![KeyCode::Char('h'), KeyCode::Left],
        },
        Keybind {
            action: Action::MenuRight,
            keys: vec![KeyCode::Char('l'), KeyCode::Right],
        },
        Keybind {
            action: Action::MenuSelect,
            keys: vec![KeyCode::Enter, KeyCode::Char(' ')],
        },
        Keybind {
            action: Action::MenuBack,
            keys: vec![KeyCode::Esc, KeyCode::Char('q')],
        },
    ]
}

/// Keybind resolver.
pub struct KeybindMap {
    binds: Vec<Keybind>,
}

impl KeybindMap {
    pub fn new() -> Self {
        Self {
            binds: default_keybinds(),
        }
    }

    /// Look up the action for a key code in game context.
    pub fn resolve_game(&self, key: KeyCode) -> Option<Action> {
        for bind in &self.binds {
            if bind.keys.contains(&key) {
                match bind.action {
                    // Only return game actions during play
                    Action::MoveLeft
                    | Action::MoveRight
                    | Action::SoftDrop
                    | Action::HardDrop
                    | Action::RotateCW
                    | Action::RotateCCW
                    | Action::Rotate180
                    | Action::Hold
                    | Action::Pause
                    | Action::Quit
                    | Action::Restart => return Some(bind.action),
                    _ => {}
                }
            }
        }
        None
    }

    /// Look up the action for a key code in menu context.
    pub fn resolve_menu(&self, key: KeyCode) -> Option<Action> {
        for bind in &self.binds {
            if bind.keys.contains(&key) {
                match bind.action {
                    Action::MenuUp
                    | Action::MenuDown
                    | Action::MenuLeft
                    | Action::MenuRight
                    | Action::MenuSelect
                    | Action::MenuBack
                    | Action::Quit => return Some(bind.action),
                    _ => {}
                }
            }
        }
        None
    }

    /// Get the display string for an action's primary key.
    pub fn key_display(&self, action: Action) -> String {
        for bind in &self.binds {
            if bind.action == action {
                if let Some(key) = bind.keys.first() {
                    return format_key(*key);
                }
            }
        }
        "?".to_string()
    }
}

/// Format a key code for display.
pub fn format_key(key: KeyCode) -> String {
    match key {
        KeyCode::Char(' ') => "Space".to_string(),
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Left => "←".to_string(),
        KeyCode::Right => "→".to_string(),
        KeyCode::Up => "↑".to_string(),
        KeyCode::Down => "↓".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Backspace => "Bksp".to_string(),
        _ => "?".to_string(),
    }
}
