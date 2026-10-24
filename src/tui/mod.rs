//! The TUI frame shared by every view: shell (theme + keys), tab bar,
//! status bar and panes.

mod pane;
mod scene;
mod select_list;
mod shell;
mod status_bar;
mod tab_bar;

pub use pane::Pane;
pub use scene::Scene;
pub use select_list::{focus_follows, reveal, ListRow, SelectList};
pub use shell::{current_mode, follow, on_intent, save_theme, Selections, Targets, TuiShell, ViewKind};
pub use status_bar::StatusBar;
pub use tab_bar::TabBar;
