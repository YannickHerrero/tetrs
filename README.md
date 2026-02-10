# tetrs

A modern terminal-first Tetris clone written in Rust.

`tetrs` uses guideline-inspired mechanics (SRS kicks, T-spin detection, 7-bag randomizer, hold, ghost piece), multiple modes, and a polished TUI built with `ratatui` + `crossterm`.

## Highlights

- Full single-board Tetris engine with gravity, lock delay, scoring, combos, back-to-back, and perfect clears.
- Multiple modes: `40 Lines Sprint`, `Endless Marathon`, and `Versus AI`.
- AI opponent with four difficulty presets: `Easy`, `Medium`, `Hard`, `Expert`.
- Fast keyboard input with DAS/ARR handling and Vim-friendly defaults.
- Animated terminal UI with sidebars, effects, action text, and game over screens.
- Local high score persistence via JSON.

## Tech Stack

- Rust 2021
- `ratatui` for rendering
- `crossterm` for terminal/input
- `serde` + `serde_json` for persistence

## Quick Start

### Requirements

- Rust (stable toolchain)
- A terminal with at least:
  - `56x26` for single-player modes
  - `106x26` for full side-by-side versus layout

### Run

```bash
cargo run --release
```

### Test

```bash
cargo test
```

## Controls

Default controls are Vim-style, with arrow key alternatives for movement/navigation.

### In Game

| Action | Keys |
| --- | --- |
| Move left | `h`, `Left` |
| Move right | `l`, `Right` |
| Soft drop | `j`, `Down` |
| Hard drop | `k`, `Up`, `Space` |
| Rotate CW | `f`, `x` |
| Rotate CCW | `d`, `z` |
| Rotate 180 | `s`, `a` |
| Hold | `g`, `c` |
| Pause | `Esc`, `p` |
| Restart | `r` |
| Quit to menu | `q` |

### Menus

| Action | Keys |
| --- | --- |
| Navigate | `h/j/k/l` or arrow keys |
| Select | `Enter`, `Space` |
| Back | `Esc`, `q` |

## Game Modes

- `40 Lines Sprint`: clear 40 lines as fast as possible.
- `Endless Marathon`: survive and maximize score.
- `Versus AI`: battle an AI board with garbage exchange and selectable difficulty.

## Persistence

High scores are stored as JSON under your OS config directory:

- Linux: `~/.config/tetrs/high_scores.json`
- macOS: `~/Library/Application Support/tetrs/high_scores.json`
- Windows: `%APPDATA%\\tetrs\\high_scores.json`

## Project Layout

```text
src/
  app.rs            # App state machine and screen flow
  game/             # Core tetris engine (board, pieces, SRS, scoring, garbage)
  modes/            # Sprint, Endless, Versus mode wrappers
  ai/               # Heuristic AI and difficulty presets
  input/            # Key mapping + DAS/ARR handling
  ui/               # Ratatui screens, widgets, layout, effects
  data/             # JSON-backed persistence
```

## Notes

- The app runs in an alternate screen and restores your terminal on exit/panic.
- If your terminal is too small, tetrs shows a size warning instead of rendering a broken layout.
