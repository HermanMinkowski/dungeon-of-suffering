# AGENTS.md - Dungeon of Suffering AI Guide

## Project Overview

**Dungeon of Suffering** is a text adventure game written in Rust, playable as both a CLI and WebAssembly application. The project emphasizes internationalization (i18n), state-driven game flow, and natural language processing.

## Architecture

### Core Game Loop Pattern: State Machine

The game uses a **functional state machine pattern** (`State<T>`) defined in `src/state.rs`:

```rust
pub type StateFn<T> = fn(&mut T) -> State<T>;
pub struct State<T> {
    function: StateFn<T>,           // Next state function to execute
    requires_input: bool,            // Whether to prompt player
    completed: bool,                 // Game over flag
    output: String,                  // Text to display
}
```

**Key point**: Game flow is a chain of function pointers. Each state function (e.g., `Game::inn()`, `Game::cave_entrance()`) returns the next state. See `src/game/village.rs` for examples.

### Module Organization

- **`cli.rs`** - Entry point; manages game loop, input parsing, language loading
- **`game.rs`** - Game struct; aggregates player, doors, equipment, vocabulary, i18n
- **`game/{village,cave_entrance,cave_water,cave_mimic}.rs`** - Impl blocks adding scene functions to `Game`
- **`vocabulary/`** - Language-aware command/verb/object parsing via i18n
- **`player.rs`** - Player state (name, equipment inventory)
- **`equipment.rs`** - Item system with quantities and translation keys
- **`doors.rs`** - Door enum for lock state tracking
- **`i18n.rs`** - Fluent-based translation wrapper

### Data Flow

```
CLI input → normalize + parse → ParsedInput (verb, object, command)
                                      ↓
                           Current state function processes input
                                      ↓
                           Game state mutates (doors, inventory, etc.)
                                      ↓
                           Next State returned with text output
```

## Build & Test Workflow

### Commands

```bash
# Development (debug binary)
cargo build

# WebAssembly target
cargo build --target wasm32-unknown-unknown

# Run the game
cargo run

# Run tests
cargo test
```

**Note**: `Cargo.toml` uses `edition = "2024"` (non-standard; verify compatibility).

## Critical Patterns & Conventions

### State Functions Return Pattern

State functions always mutate `&mut Game` and return `State<Game>`. Use helpers to create states:

```rust
// From village.rs
pub fn inn(&mut self) -> State<Game> {
    let text = self.text("intro.text");
    State::with_input(Self::help, text)  // Next state needs input
}

// Helpers in state.rs
State::no_input(func, output)      // Continue without prompting
State::with_input(func, output)    // Prompt for next command
State::completed(func)             // Game over
```

### Internationalization (i18n) Integration

- All user-facing text is stored in **Fluent** files (`locales/{en,fr}.ftl`)
- Access via `self.i18n.t(key, args)` or `self.text(key)` helper
- **Vocabulary is dynamic**: `Commands`, `Verbs`, `Objects` are refreshed per language change
- `src/i18n.rs` handles Fluent bundle initialization; `src/vocabulary/commands.rs` shows parsing pattern

**Example pattern** (from `commands.rs`):
```rust
pub struct Commands {
    help: String,    // Localized string
    equipment: String,
}

pub fn parse(&self, input: &str) -> Command {
    if input == self.help { Command::Help }
    // ... match against localized strings
}
```

### Input Normalization

`cli.rs::normalize()` removes diacritics and lowercases input:
```rust
pub fn normalize(s: &str) -> String {
    s.nfd()
        .filter(|c| !is_combining_mark(*c))
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
}
```

Use this when comparing user input to vocabulary keywords.

### Equipment & Item System

Items have **quantities** (consumables like gold) with caps:
```rust
pub struct Item {
    kind: ItemKind,
    amount: u32,        // Current quantity
    max_amount: u32,    // Capacity limit
}
```

Item kinds map to translation keys: `ItemKind::Sword` → `"object.sword"` key in FTL.

### Global Commands

Scenes call `self.handle_global_commands(command)` to check for `Help`, `Equipment`, `Quit` (defined in `src/game.rs`). Return early if matched.

## File Dependencies & Data Flows

```
main.rs
  └─> cli.rs (CLI entry, game loop)
       ├─> game.rs (Game struct, aggregates subsystems)
       │   ├─> game/*.rs (scene implementations via impl blocks)
       │   ├─> player.rs (name, inventory)
       │   ├─> equipment.rs (item types, quantities)
       │   ├─> doors.rs (lock tracking)
       │   ├─> vocabulary/* (localized parsing)
       │   └─> i18n.rs (Fluent wrapper)
       ├─> i18n.rs (translation loading)
       └─> vocabulary/* (command/verb/object parsing)

lib.rs exposes: cli, doors, equipment, game, player, vocabulary
```

## Language Files

- `locales/en.ftl` and `locales/fr.ftl` - Fluent format (parametric messages)
- `locales/en.json` and `locales/fr.json` - Flat translation objects
- FTL format supports interpolation: `"message" = "Hello {$name}"`

Load language at CLI init (currently hardcoded to French in `cli.rs::default()`).

## WASM Build Quirks

- Project compiles to `target/wasm32-unknown-unknown/release/`
- Bindings: `dungeon-of-suffering.js`, `dungeon-of-suffering_bg.wasm`
- See `web/` directory for artifacts

## Dependencies Overview

- **rust-i18n** - Macro-based i18n (less common; project uses Fluent instead)
- **fluent-bundle** - Core translation engine
- **strum/strum_macros** - Enum iteration (used for `Doors`)
- **serde** - JSON serialization (used for config/items)
- **wasm-bindgen** - WASM FFI bindings
- **unicode-normalization** - Input diacritical stripping

## Conventions for New Code

1. **Scene functions** go in `src/game/{scene}.rs` as impl blocks on `Game`
2. **New items** add to `ItemKind` enum + translation key mapping in `equipment.rs`
3. **Text output** always uses i18n keys; add to FTL files, never hardcode strings
4. **Input parsing** normalizes + compares against localized vocabulary
5. **State returns** use `State` helpers; always return next function pointer
6. **Mutation** happens on `&mut self` (Game); states are pure logic functions

