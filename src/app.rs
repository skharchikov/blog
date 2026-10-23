use crate::tui::TuiShell;
use crate::views::{BooksView, ContactView, HomeView, NotFoundView, PostView, ProjectView, ProjectsView, WritingView};
use leptos::*;
use leptos_router::{TrailingSlash, *};

const HIGHLIGHT_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/highlight.css"));
const THEMES_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/themes.css"));

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <style inner_html=THEMES_CSS></style>
            <style inner_html=HIGHLIGHT_CSS></style>
            <TuiShell>
                <Routes>
                    <Route path="/" view=HomeView />
                    <Route path="/posts" view=WritingView />
                    <Route path="/posts/:slug" view=PostView trailing_slash=TrailingSlash::Redirect />
                    <Route path="/projects" view=ProjectsView />
                    <Route path="/projects/:slug" view=ProjectView trailing_slash=TrailingSlash::Redirect />
                    <Route path="/books" view=BooksView trailing_slash=TrailingSlash::Redirect />
                    <Route path="/contacts" view=ContactView trailing_slash=TrailingSlash::Redirect />
                    <Route path="/*any" view=NotFoundView />
                </Routes>
            </TuiShell>
        </Router>
    }
}
