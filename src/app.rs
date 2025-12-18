use leptos::*;
use leptos_router::*;
use crate::components::{Nav, PostList, PostView, About};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app-container">
                <Nav />
                <main class="main-content">
                    <Routes>
                        <Route path="/" view=PostList />
                        <Route path="/about" view=About />
                        <Route path="/post/:slug" view=PostView />
                    </Routes>
                </main>
                <footer class="footer">
                    <p>"Built with Leptos 🦀"</p>
                </footer>
            </div>
        </Router>
    }
}
