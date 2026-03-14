# Migration from rust-i18n to Fluent

## Summary

Successfully converted the Dungeon of Suffering project from the `rust-i18n` macro-based system to a `fluent-bundle` based system.

## Changes Made

### 1. Dependencies (Cargo.toml)
- **Removed**: `rust-i18n = "3"`
- **Added**: 
  - `fluent-bundle = "0.15"`
  - `unic-langid = "0.9"`

### 2. Core i18n Module (src/i18n.rs)
- Already using Fluent, kept implementation
- Added helper methods:
  - `I18n::with_english()` - create English instance
  - `I18n::with_french()` - create French instance

### 3. Game Struct (src/game.rs)
- Added `pub i18n: I18n` field to Game
- Removed `use rust_i18n::t;` import
- Updated all `t!()` macro calls to use `self.i18n.t(key, args)` method
- Added `use fluent_bundle::FluentArgs;` for passing arguments to translations
- Updated `Game::default()` to initialize I18n before Vocabulary
- Updated `Equipment::list()` calls to pass `&self.i18n` reference

### 4. Equipment (src/equipment.rs)
- Removed `use rust_i18n::t;`
- Added `use crate::i18n::I18n;`
- Updated `Equipment::list()` method signature to accept `i18n: &I18n` parameter
- Replaced `t!()` macro calls with `i18n.t()` method calls

### 5. Vocabulary System

#### Commands (src/vocabulary/commands.rs)
- Removed `use rust_i18n::t;`
- Added `use crate::i18n::I18n;`
- Updated `Commands::new()` to accept `i18n: &I18n` parameter
- Updated `Commands::refresh()` to accept `i18n: &I18n` parameter
- Replaced all `t!()` calls with `i18n.t()`

#### Verbs (src/vocabulary/verbs.rs)
- Removed `use rust_i18n::t;`
- Added `use crate::i18n::I18n;`
- Updated `Verbs::new()` to accept `i18n: &I18n` parameter
- Updated `Verbs::refresh()` to accept `i18n: &I18n` parameter
- Replaced all `t!()` calls with `i18n.t()`

#### Objects (src/vocabulary/objects.rs)
- Removed `use rust_i18n::t;`
- Added `use crate::i18n::I18n;`
- Updated `Objects::new()` to accept `i18n: &I18n` parameter
- Updated `Objects::refresh()` to accept `i18n: &I18n` parameter
- Replaced all `t!()` calls with `i18n.t()`

#### Vocabulary (src/vocabulary/mod.rs)
- Added `use crate::i18n::I18n;`
- Updated `Vocabulary::new()` to accept `i18n: &I18n` parameter
- Updated `Vocabulary::refresh()` to accept `i18n: &I18n` parameter
- Now properly passes I18n to all vocabulary components

### 6. CLI (src/cli.rs)
- Removed `use crate::vocabulary::Vocabulary;` (now uses game's vocabulary)
- Removed duplicate `vocabulary: Vocabulary` field from Cli struct
- Removed `rust_i18n::set_locale("fr");` call from game_loop
- Removed `self.vocabulary.refresh();` call
- Updated `generate_parsed_input()` to use `self.game.vocabulary` instead of `self.vocabulary`

### 7. Main (src/main.rs)
- Removed `extern crate rust_i18n;` declaration

### 8. Library (src/lib.rs)
- Removed `use rust_i18n::i18n;` import
- Removed `i18n!("locales", fallback = "en");` macro call
- Added `pub mod i18n;` to exports

## How It Works

### Translation Lookup
```rust
// Without arguments
self.i18n.t("title", None)

// With arguments
let mut args = FluentArgs::new();
args.set("name", "Player Name");
self.i18n.t("message.end", Some(&args))
```

### Language Support
The I18n module supports French (default) and English:
```rust
let i18n_en = I18n::with_english();
let i18n_fr = I18n::with_french();
```

### Vocabulary Initialization
Vocabulary instances are now initialized with an I18n reference:
```rust
let i18n = I18n::new("fr", include_str!("../locales/fr.ftl"));
let vocabulary = Vocabulary::new(&i18n);
```

## Files Modified
1. Cargo.toml
2. src/lib.rs
3. src/main.rs
4. src/i18n.rs
5. src/game.rs
6. src/equipment.rs
7. src/cli.rs
8. src/vocabulary/mod.rs
9. src/vocabulary/commands.rs
10. src/vocabulary/verbs.rs
11. src/vocabulary/objects.rs

## Notes
- Locale files (locales/en.ftl, locales/fr.ftl) have been converted from JSON to proper Fluent format
  - Keys converted from dot notation (e.g., `object.bread`) to hyphenated notation (e.g., `object-bread`)
  - Parameters converted from `%{var}` syntax to proper Fluent `{ $var }` syntax
- The Fluent format is preserved with full interpolation support
- Scene files (game/village.rs, etc.) require no changes as they use `self.text()` helper method
- wasm-bindgen dependencies unchanged
- Build verification: Project compiles successfully with no warnings or errors

