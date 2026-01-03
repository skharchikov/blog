use crate::components::{Contacts, Home, Nav, PostList, PostView, ProjectView, Projects};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app-container">
                <div class="corner-tr"></div>
                <div class="corner-bl"></div>
                <div class="corner-br"></div>

                <Nav />
                <main class="main-content">
                    <Routes>
                        <Route path="/" view=Home />
                        <Route path="/projects" view=Projects />
                        <Route path="/projects/:slug" view=ProjectView />
                        <Route path="/posts" view=PostList />
                        <Route path="/posts/:slug" view=PostView />
                        <Route path="/contacts" view=Contacts />
                    </Routes>
                </main>
                <footer class="footer">
                    <p>"Built with Leptos 🦀"</p>
                </footer>
                // <XmasTree />
            </div>
        </Router>
    }
}
