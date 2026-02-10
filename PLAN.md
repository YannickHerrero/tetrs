# Tetrs - Implementation Plan

## Project Overview

A modern terminal-based Tetris game in Rust with full guideline mechanics, multiple game modes, AI versus mode, and polished TUI visuals. Built with **ratatui + crossterm**, persisting high scores via **JSON files**.

## Architecture

```
tetrs/
├── Cargo.toml
├── src/
│   ├── main.rs                  # Entry point, terminal setup, panic hook
│   ├── app.rs                   # App state machine
│   ├── game/                    # Core game logic
│   │   ├── mod.rs, board.rs, piece.rs, srs.rs, movement.rs
│   │   ├── gravity.rs, locking.rs, hold.rs, bag.rs, ghost.rs
│   │   ├── garbage.rs, scoring.rs, stats.rs, clear.rs
│   ├── modes/                   # Game modes
│   │   ├── mod.rs, sprint.rs, endless.rs, versus.rs
│   ├── ai/                      # AI opponent
│   │   ├── mod.rs, evaluator.rs, placement.rs, difficulty.rs
│   ├── ui/                      # Rendering & UI
│   │   ├── mod.rs, theme.rs, layout.rs, effects.rs
│   │   ├── widgets/ (board, sidebar, next_queue, hold_box, garbage_bar, action_text)
│   │   ├── screens/ (menu, game, game_over, high_scores)
│   ├── input/                   # Input handling
│   │   ├── mod.rs, das.rs, keybinds.rs
│   └── data/                    # Persistence
│       ├── mod.rs, high_scores.rs, config.rs
```

## Implementation Order

1. Project setup + piece definitions
2. SRS kicks + board with collision
3. Movement, rotation, T-spin detection, gravity, locking
4. Hold, bag randomizer, ghost piece
5. Scoring, attack, garbage, stats
6. Input system with DAS/ARR and Vim keybinds
7. Theme, layout, board widget, sidebar widgets, effects
8. Menu, game over, high score screens
9. Sprint mode, endless mode
10. AI evaluator, move gen, difficulty
11. Versus mode (integrates AI + garbage)
12. High score and config persistence
13. Polish pass, performance tuning, visual refinement
