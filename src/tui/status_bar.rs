use super::shell::{save_theme, ViewKind};
use leptos::*;
use tui_core::keys::View;
use tui_core::theme::Theme;

/// Key hints for the current view. The hints repeat what visible controls do,
/// so they are `aria-hidden`; the theme switcher is a real button outside them.
#[component]
pub fn StatusBar() -> impl IntoView {
    let view_kind = expect_context::<ViewKind>().0;
    let hints = move || {
        let keys: &[(&str, &str)] = match view_kind.get() {
            View::Home => &[("help", "commands"), ("esc", "leave shell"), ("1-5", "tabs")],
            View::List => &[("1-5", "tabs"), ("j/k", "move"), ("enter", "open"), ("/", "filter")],
            View::Grid => &[("1-5", "tabs"), ("h j k l", "move"), ("enter", "open")],
            View::Reader => &[("1-5", "tabs"), ("q", "back")],
            View::Plain => &[("1-5", "tabs")],
        };
        let mode = if view_kind.get() == View::Home { "SHELL" } else { "NORMAL" };
        view! {
            <span class="mode">{mode}</span>
            {keys
                .iter()
                .map(|(k, what)| view! { <span class="hint"><span class="k">{*k}</span>{*what}</span> })
                .collect_view()}
            <span class="hint"><span class="k">"t"</span>"theme"</span>
        }
    };
    view! {
        <footer class="status">
            <div class="hints" aria-hidden="true">{hints}</div>
            <ThemeButton />
        </footer>
    }
}

#[component]
fn ThemeButton() -> impl IntoView {
    let theme = expect_context::<RwSignal<Theme>>();
    let cycle = move |_| {
        let next = theme.get_untracked().next();
        theme.set(next);
        save_theme(next);
    };
    view! {
        <button class="theme-btn" on:click=cycle aria-label=move || format!("Theme: {}. Switch theme", theme.get().name())>
            {move || format!("theme: {} ▸", theme.get().name())}
        </button>
    }
}
