use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav() -> impl IntoView {
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

    view! {
        <nav class="navbar">
            <div class="nav-content">
                <div class="nav-left">
                    <A
                        href={back_link}
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
