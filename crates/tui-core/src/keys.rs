//! Keyboard handling: what a key press means in the current view.

use crate::tabs::Tab;

/// `Typing` while focus is in a text field, editable element or iframe: every
/// key belongs to that element, so no shortcut fires.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Typing,
}

/// The kind of view that has the keyboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View {
    Home,
    List,
    Grid,
    Reader,
    Plain,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intent {
    Tab(Tab),
    Move(i32),
    MoveGrid { dx: i32, dy: i32 },
    Open,
    Back,
    CycleTheme,
    FocusFilter,
    OpenGithub,
    None,
}

/// Map a `KeyboardEvent.key` to an intent. Shortcuts never take modifiers, so
/// browser shortcuts like Ctrl+1 keep working.
pub fn intent(key: &str, ctrl_alt_meta: bool, mode: Mode, view: View) -> Intent {
    if mode == Mode::Typing || ctrl_alt_meta {
        return Intent::None;
    }
    match (view, key) {
        (_, "1" | "2" | "3" | "4" | "5") => {
            key.parse().ok().and_then(Tab::from_number).map_or(Intent::None, Intent::Tab)
        }
        (_, "t") => Intent::CycleTheme,
        (View::List, "j" | "ArrowDown") => Intent::Move(1),
        (View::List, "k" | "ArrowUp") => Intent::Move(-1),
        (View::List, "/") => Intent::FocusFilter,
        (View::List, "g") => Intent::OpenGithub,
        (View::Grid, "h" | "ArrowLeft") => Intent::MoveGrid { dx: -1, dy: 0 },
        (View::Grid, "l" | "ArrowRight") => Intent::MoveGrid { dx: 1, dy: 0 },
        (View::Grid, "k" | "ArrowUp") => Intent::MoveGrid { dx: 0, dy: -1 },
        (View::Grid, "j" | "ArrowDown") => Intent::MoveGrid { dx: 0, dy: 1 },
        (View::List | View::Grid, "Enter") => Intent::Open,
        (View::Reader, "q" | "Escape") => Intent::Back,
        _ => Intent::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tabs::Tab;

    #[test]
    fn typing_never_triggers() {
        for k in ["1", "5", "t", "q", "j", "k", "Enter", "/", "g", "Escape"] {
            assert_eq!(intent(k, false, Mode::Typing, View::List), Intent::None);
        }
    }

    #[test]
    fn modifiers_never_trigger() {
        assert_eq!(intent("1", true, Mode::Normal, View::List), Intent::None);
    }

    #[test]
    fn tabs_and_theme() {
        assert_eq!(intent("3", false, Mode::Normal, View::Plain), Intent::Tab(Tab::Projects));
        assert_eq!(intent("t", false, Mode::Normal, View::Plain), Intent::CycleTheme);
        assert_eq!(intent("6", false, Mode::Normal, View::Plain), Intent::None);
    }

    #[test]
    fn list_keys() {
        assert_eq!(intent("j", false, Mode::Normal, View::List), Intent::Move(1));
        assert_eq!(intent("ArrowUp", false, Mode::Normal, View::List), Intent::Move(-1));
        assert_eq!(intent("Enter", false, Mode::Normal, View::List), Intent::Open);
        assert_eq!(intent("/", false, Mode::Normal, View::List), Intent::FocusFilter);
        assert_eq!(intent("g", false, Mode::Normal, View::List), Intent::OpenGithub);
    }

    #[test]
    fn grid_keys() {
        assert_eq!(intent("l", false, Mode::Normal, View::Grid), Intent::MoveGrid { dx: 1, dy: 0 });
        assert_eq!(intent("h", false, Mode::Normal, View::Grid), Intent::MoveGrid { dx: -1, dy: 0 });
        assert_eq!(intent("k", false, Mode::Normal, View::Grid), Intent::MoveGrid { dx: 0, dy: -1 });
        assert_eq!(intent("ArrowDown", false, Mode::Normal, View::Grid), Intent::MoveGrid { dx: 0, dy: 1 });
    }

    #[test]
    fn back() {
        assert_eq!(intent("q", false, Mode::Normal, View::Reader), Intent::Back);
        assert_eq!(intent("Escape", false, Mode::Normal, View::Reader), Intent::Back);
        assert_eq!(intent("q", false, Mode::Normal, View::Home), Intent::None);
    }
}
