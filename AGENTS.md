# Repository Guidelines

## Project Structure & Module Organization

This is a Rust 2024 Bevy platformer crate named `bigkidsuperhero_platformer`. Source lives under `src/`, with `src/main.rs` wiring Bevy plugins, states, and systems. Keep ECS data types in `src/components/`, gameplay systems in `src/gameplay/`, UI/menu flows in `src/screens/`, shared systems in `src/systems/`, audio setup in `src/audio/`, and level definitions/spawn data in `src/levels.rs`.

Runtime assets are in `assets/`. Character art is grouped under `assets/bridget/` and `assets/calvin/`; imported packs keep their original folders, such as `assets/Pixel Adventure 1/` and `assets/brackeys_platformer_assets/`. Utility scripts for sprite processing live in `scripts/`.

## Build, Test, and Development Commands

- `cargo run` builds and launches the game locally.
- `cargo run --features dev` enables Bevy dynamic linking for faster local rebuilds.
- `cargo check` verifies the crate without producing a runnable binary.
- `cargo test` runs Rust unit and integration tests.
- `cargo fmt` formats Rust code using rustfmt.
- `cargo clippy --all-targets --all-features` checks for common Rust issues.

The local `.cargo/config.toml` targets Linux with `clang` and `mold`; install those tools or adjust the config if your platform differs.

## Coding Style & Naming Conventions

Use standard Rust formatting: four-space indentation, `snake_case` modules/functions/fields, `PascalCase` types and Bevy components/resources, and `SCREAMING_SNAKE_CASE` constants. Prefer Bevy ECS patterns already used in `main.rs`: small systems, explicit state guards with `run_if`, and cleanup systems on state exits. Keep asset paths stable and relative to the repository root.

## Testing Guidelines

There is no dedicated `tests/` directory yet. Add focused unit tests beside the code they cover using `#[cfg(test)] mod tests`, or integration tests under `tests/` when behavior crosses modules. Prioritize deterministic tests for level data, collision helpers, persistence, scoring, and state transitions. Run `cargo test` and `cargo clippy --all-targets --all-features` before submitting gameplay logic changes.

## Commit & Pull Request Guidelines

Recent commits use short imperative subjects, for example `Fix player sprite alignment` and `Use Pixel Adventure level assets`. Follow that style: concise, present-tense, and specific. Pull requests should describe gameplay or asset impact, list commands run, link related issues, and include screenshots or short clips for visual changes. Call out asset source/licensing changes and any save-file compatibility impact.

## Agent-Specific Instructions

Do not overwrite unrelated local changes. This repository may contain in-progress asset and player edits; inspect `git status --short` before modifying files, and keep changes scoped to the requested task.
