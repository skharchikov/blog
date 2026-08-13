use crate::components::{
    Bookshelf, Contacts, Home, Nav, PostList, PostView, ProjectView, Projects,
};
use leptos::*;
use leptos_router::{TrailingSlash, *};
use web_sys::window;

const HIGHLIGHT_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/highlight.css"));

#[component]
pub fn App() -> impl IntoView {
    let initial_dark_mode = window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item("darkMode").ok().flatten())
        .map(|v| v == "true")
        .unwrap_or(false);

    let (dark_mode, set_dark_mode) = create_signal(initial_dark_mode);
    provide_context(dark_mode);
    provide_context(set_dark_mode);

    create_effect(move |_| {
        let is_dark = dark_mode.get();
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    if is_dark {
                        let _ = body.class_list().add_1("dark-mode");
                    } else {
                        let _ = body.class_list().remove_1("dark-mode");
                    }
                }
            }
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("darkMode", if is_dark { "true" } else { "false" });
            }
        }
    });

    view! {
        <Router>
            <style inner_html=HIGHLIGHT_CSS></style>
            <div class="app-container">
                <div class="corner-tl"></div>
                <div class="corner-tr"></div>
                <div class="corner-bl"></div>
                <div class="corner-br"></div>

                <Nav />
                <main class="main-content">
                    <Routes>
                        <Route path="/" view=Home />
                        <Route path="/projects" view=Projects />
                        <Route path="/projects/:slug" view=ProjectView trailing_slash=TrailingSlash::Redirect />
                        <Route path="/posts" view=PostList />
                        <Route path="/posts/:slug" view=PostView trailing_slash=TrailingSlash::Redirect />
                        <Route path="/books" view=Bookshelf trailing_slash=TrailingSlash::Redirect />
                        <Route path="/contacts" view=Contacts />
                    </Routes>
                </main>
                <footer class="footer">
                    <a href="https://leptos.dev" target="_blank" rel="noopener noreferrer">{"Built with Leptos 🦀"}</a>
                </footer>
                // <XmasTree />
            </div>
        </Router>
    }
}
