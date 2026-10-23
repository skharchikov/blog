use super::{StatusBar, TabBar};
use leptos::*;
use leptos_router::*;
use tui_core::keys::{intent, Intent, Mode, View};
use tui_core::theme::Theme;

/// The most recent key intent a view hasn't handled yet. Views read it, act,
/// and reset it to `None`.
#[derive(Clone, Copy)]
pub struct KeyBus(pub RwSignal<Option<Intent>>);

/// The kind of view on screen, set by each view on mount; decides what keys mean.
#[derive(Clone, Copy)]
pub struct ViewKind(pub RwSignal<View>);

/// `Typing` while focus is in a text field, editable element or iframe
/// (giscus), so shortcuts never steal keystrokes.
pub fn current_mode() -> Mode {
    let Some(el) = document().active_element() else { return Mode::Normal };
    let tag = el.tag_name().to_ascii_lowercase();
    if matches!(tag.as_str(), "input" | "textarea" | "select" | "iframe") || el.has_attribute("contenteditable") {
        Mode::Typing
    } else {
        Mode::Normal
    }
}

/// Persist an explicit theme choice. Storage can be blocked (private mode);
/// then the choice just lasts for this visit.
pub fn save_theme(theme: Theme) {
    if let Some(storage) = window().local_storage().ok().flatten() {
        let _ = storage.set_item("theme", theme.name());
    }
}

/// The theme the inline `<head>` script already applied before first paint.
fn initial_theme() -> Theme {
    document()
        .document_element()
        .and_then(|el| el.get_attribute("data-theme"))
        .and_then(|name| Theme::from_name(&name))
        .unwrap_or(Theme::Skh)
}

/// The TUI frame: tab bar, the routed view, status bar. Owns the theme and
/// the global keyboard listener.
#[component]
pub fn TuiShell(children: Children) -> impl IntoView {
    let theme = create_rw_signal(initial_theme());
    let bus = KeyBus(create_rw_signal(None));
    let view_kind = ViewKind(create_rw_signal(View::Plain));
    provide_context(theme);
    provide_context(bus);
    provide_context(view_kind);

    create_effect(move |_| {
        if let Some(el) = document().document_element() {
            let _ = el.set_attribute("data-theme", theme.get().name());
        }
    });

    let navigate = use_navigate();
    window_event_listener(ev::keydown, move |e| {
        let modifiers = e.ctrl_key() || e.alt_key() || e.meta_key();
        match intent(&e.key(), modifiers, current_mode(), view_kind.0.get_untracked()) {
            Intent::None => {}
            Intent::Tab(tab) => {
                e.prevent_default();
                navigate(tab.path(), Default::default());
            }
            Intent::CycleTheme => {
                let next = theme.get_untracked().next();
                theme.set(next);
                save_theme(next);
            }
            other => {
                e.prevent_default();
                bus.0.set(Some(other));
            }
        }
    });

    view! {
        <a class="skip" href="#main">"skip to content"</a>
        <div class="frame">
            <TabBar />
            <main id="main" class="body">{children()}</main>
            <StatusBar />
        </div>
    }
}
