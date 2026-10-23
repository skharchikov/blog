use super::{StatusBar, TabBar};
use leptos::*;
use wasm_bindgen::JsCast;
use tui_core::keys::{intent, Intent, Mode, View};
use tui_core::theme::Theme;

/// Key intents for the current view, as `(sequence number, intent)`. The
/// number grows with every key press, so a repeated intent (j, j) is still a
/// new value and views never have to write the signal back.
#[derive(Clone, Copy)]
pub struct KeyBus(pub RwSignal<(u64, Intent)>);

/// The kind of view on screen, set by each view on mount; decides what keys mean.
#[derive(Clone, Copy)]
pub struct ViewKind(pub RwSignal<View>);

/// Where Enter / q / g lead in the current view. Views keep these up to date;
/// the shell navigates from its keydown handler, because the router only
/// completes navigations started from an event handler (not from an effect).
/// Site paths go through the router, `http…` URLs open in a new tab.
#[derive(Clone, Copy)]
pub struct Targets {
    pub open: RwSignal<Option<String>>,
    pub back: RwSignal<Option<String>>,
    pub github: RwSignal<Option<String>>,
}

impl Targets {
    /// Publish this view's targets (all three, so nothing stale survives from
    /// the previous view). Intents are gated by the view kind, so views that
    /// never bind can't trigger a stale target.
    pub fn bind(open: impl Fn() -> Option<String> + 'static, back: Option<&'static str>, github: impl Fn() -> Option<String> + 'static) {
        let t = expect_context::<Targets>();
        create_effect(move |_| t.open.set(open()));
        create_effect(move |_| t.github.set(github()));
        t.back.set(back.map(String::from));
    }
}

/// Navigate the way a mouse click does: activate a real link, so the router's
/// own anchor handling runs (programmatic `navigate` waits on pending
/// resources and can lag). `http…` URLs open in a new tab.
pub fn follow(href: &str) {
    if href.starts_with("http") {
        let _ = window().open_with_url_and_target(href, "_blank");
        return;
    }
    let Ok(a) = document().create_element("a") else { return };
    let _ = a.set_attribute("href", href);
    if let (Some(body), Ok(a)) = (document().body(), a.dyn_into::<web_sys::HtmlElement>()) {
        let _ = body.append_child(&a);
        a.click();
        a.remove();
    }
}

/// Run `f` for every key intent the shell hands to the current view after it
/// mounted (the first run only records the current sequence number, so a new
/// view never replays an old key).
pub fn on_intent(f: impl Fn(Intent) + 'static) {
    let bus = expect_context::<KeyBus>().0;
    create_effect(move |seen: Option<u64>| {
        let (n, intent) = bus.get();
        if seen.is_some_and(|s| s != n) {
            f(intent);
        }
        n
    });
}

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
    let bus = KeyBus(create_rw_signal((0, Intent::None)));
    let view_kind = ViewKind(create_rw_signal(View::Plain));
    provide_context(theme);
    provide_context(bus);
    provide_context(view_kind);
    let targets = Targets { open: create_rw_signal(None), back: create_rw_signal(None), github: create_rw_signal(None) };
    provide_context(targets);

    create_effect(move |_| {
        if let Some(el) = document().document_element() {
            let _ = el.set_attribute("data-theme", theme.get().name());
        }
    });

    let go = |target: Option<String>| target.map(|href| follow(&href)).is_some();
    window_event_listener(ev::keydown, move |e| {
        let modifiers = e.ctrl_key() || e.alt_key() || e.meta_key();
        match intent(&e.key(), modifiers, current_mode(), view_kind.0.get_untracked()) {
            Intent::None => {}
            Intent::Tab(tab) => {
                e.prevent_default();
                go(Some(tab.path().to_string()));
            }
            Intent::Open if go(targets.open.get_untracked()) => e.prevent_default(),
            Intent::Back if go(targets.back.get_untracked()) => e.prevent_default(),
            Intent::OpenGithub if go(targets.github.get_untracked()) => e.prevent_default(),
            Intent::CycleTheme => {
                let next = theme.get_untracked().next();
                theme.set(next);
                save_theme(next);
            }
            other => {
                e.prevent_default();
                bus.0.update(|(n, i)| {
                    *n += 1;
                    *i = other;
                });
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
