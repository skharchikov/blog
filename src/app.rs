use crate::tui::{Pane, TuiShell, ViewKind};
use crate::views::{HomeView, PostView, ProjectView, ProjectsView, WritingView};
use leptos::*;
use leptos_router::{TrailingSlash, *};
use tui_core::keys::View;

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
                    <Route path="/*any" view=Placeholder />
                </Routes>
            </TuiShell>
        </Router>
    }
}

/// Temporary stand-in while the views are being built (removed in Task 12).
#[component]
fn Placeholder() -> impl IntoView {
    expect_context::<ViewKind>().0.set(View::Plain);
    let path = use_location().pathname;
    view! {
        <Pane title="placeholder" focused=true>
            <p>{move || path.get()}</p>
        </Pane>
    }
}
