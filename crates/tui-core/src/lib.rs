//! Pure logic behind the skh.rs TUI. No dependencies, so it builds for wasm,
//! for `build.rs`, and for host-side unit tests.

pub mod commands;
pub mod keys;
pub mod tabs;
pub mod theme;
