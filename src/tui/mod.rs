//! The TUI frame shared by every view: shell (theme + keys), tab bar,
//! status bar and panes.

mod pane;
mod scene;
mod shell;
mod status_bar;
mod tab_bar;

pub use pane::Pane;
pub use scene::Scene;
pub use shell::{current_mode, save_theme, KeyBus, TuiShell, ViewKind};
pub use status_bar::StatusBar;
pub use tab_bar::TabBar;
