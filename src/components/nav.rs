use leptos::*;
use leptos_router::*;
use web_sys::window;

#[component]
pub fn Nav() -> impl IntoView {
    // Check if dark mode is enabled in localStorage
    let initial_dark_mode = window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item("darkMode").ok().flatten())
        .map(|v| v == "true")
        .unwrap_or(false);

    let (dark_mode, set_dark_mode) = create_signal(initial_dark_mode);

    // Apply dark mode class to body on mount and when toggled
    create_effect(move |_| {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    if dark_mode.get() {
                        let _ = body.class_list().add_1("dark-mode");
                    } else {
                        let _ = body.class_list().remove_1("dark-mode");
                    }
                }
            }
            // Save to localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                let _ =
                    storage.set_item("darkMode", if dark_mode.get() { "true" } else { "false" });
            }
        }
    });

    let toggle_dark_mode = move |_| {
        set_dark_mode.update(|mode| *mode = !*mode);
    };

    view! {
        <nav class="navbar">
            <div class="nav-content">
                <A href="/" class="nav-brand">
                    <h1>"My Rust Blog"</h1>
                </A>
                <div class="nav-links">
                    <A href="/" class="nav-link">"Home"</A>
                    <A href="/about" class="nav-link">"About"</A>
                    <button class="dark-mode-toggle" on:click=toggle_dark_mode>
                        {move || if dark_mode.get() { "☀️" } else { "🌙" }}
                    </button>
                </div>
            </div>
        </nav>
    }
}
