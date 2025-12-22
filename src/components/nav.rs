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

    let location = use_location();
    let is_home = move || location.pathname.get() == "/";

    view! {
        <nav class="navbar">
            <div class="nav-content">
                <div class="nav-left">
                    <A
                        href="/"
                        class=move || if is_home() {
                            "nav-link back-link hidden"
                        } else {
                            "nav-link back-link"
                        }
                    >
                        "← back"
                    </A>
                </div>
                <div class="nav-center">
                    <A href="/projects" class="nav-link">"projects"</A>
                    <A href="/posts" class="nav-link">"posts"</A>
                    <A href="/contacts" class="nav-link">"contacts"</A>
                </div>
                <div class="nav-right">
                    <button class="dark-mode-toggle" on:click=toggle_dark_mode>
                        "☀"
                    </button>
                </div>
            </div>
        </nav>
    }
}
