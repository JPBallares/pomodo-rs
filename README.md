# pomodo-rs

A tiny cross-platform Pomodoro timer written in Rust using the `iced` GUI toolkit.

## Requirements
- Rust: stable (edition 2024). Recommended: Rust 1.85+ via `rustup`.
- OS: Works on macOS, Linux, and Windows.

## Dependencies
- `iced = 0.13.1` with feature `smol`
- `rodio = "0.21.1`
- See `Cargo.toml` for exact versions and features.

## Local Setup
- Install Rust with `rustup` (https://rustup.rs) and update to latest stable:
  - `rustup update`
- Run the app in debug mode:
  - `cargo run`
- Build a release binary:
  - `cargo build --release`

## Development
- Quick compile/type-check:
  - `cargo check`
- Optional (if you have them installed):
  - Format: `cargo fmt`
  - Lints: `cargo clippy`

## Notes
- If you see errors related to the 2024 edition, update to the latest stable toolchain with `rustup update`.
