use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav() -> impl IntoView {
    let dark_mode = use_context::<ReadSignal<bool>>()
        .expect("dark mode ReadSignal not provided in App");
    let set_dark_mode = use_context::<WriteSignal<bool>>()
        .expect("dark mode WriteSignal not provided in App");

    let toggle_dark_mode = move |_| {
        set_dark_mode.update(|mode| *mode = !*mode);
    };

    let location = use_location();
    let is_home = move || location.pathname.get() == "/";

    let back_link = move || {
        let path = location.pathname.get();
        if path.starts_with("/projects/") {
            "/projects".to_string()
        } else if path.starts_with("/posts/") {
            "/posts".to_string()
        } else {
            "/".to_string()
        }
    };

    // Whether a top-level nav route is the current section.
    let active = move |prefix: &str| {
        let path = location.pathname.get();
        path == prefix || path.starts_with(&format!("{}/", prefix))
    };

    view! {
        <nav class="navbar">
            <div class="nav-content">
                <div class="nav-left">
                    {move || if is_home() {
                        view! { <span class="nav-home-glyph">"~"</span> }.into_view()
                    } else {
                        view! {
                            <A href={back_link} class="nav-link back-link">"← back"</A>
                        }.into_view()
                    }}
                </div>
                <div class="nav-center">
                    <A href="/projects" class=move || if active("/projects") { "nav-link current" } else { "nav-link" }>"projects"</A>
                    <A href="/posts" class=move || if active("/posts") { "nav-link current" } else { "nav-link" }>"posts"</A>
                    <A href="/books" class=move || if active("/books") { "nav-link current" } else { "nav-link" }>"books"</A>
                    <A href="/contacts" class=move || if active("/contacts") { "nav-link current" } else { "nav-link" }>"contacts"</A>
                </div>
                <div class="nav-right">
                    <button
                        class="dark-mode-toggle"
                        aria-label="Toggle dark mode"
                        attr:aria-pressed={move || if dark_mode.get() { "true" } else { "false" }}
                        on:click=toggle_dark_mode
                    >
                        {move || if dark_mode.get() { "☾" } else { "☀" }}
                    </button>
                </div>
            </div>
        </nav>
    }
}
