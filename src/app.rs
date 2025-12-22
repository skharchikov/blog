use leptos::*;
use leptos_router::*;
use crate::components::{Nav, Home, Projects, PostList, PostView, Contacts, XmasTree};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app-container">
                <div class="snow-container">
                    <div class="snowflake">"❄"</div>
                    <div class="snowflake">"❅"</div>
                    <div class="snowflake">"❆"</div>
                    <div class="snowflake">"❄"</div>
                    <div class="snowflake">"❅"</div>
                    <div class="snowflake">"❆"</div>
                    <div class="snowflake">"❄"</div>
                    <div class="snowflake">"❅"</div>
                    <div class="snowflake">"❆"</div>
                    <div class="snowflake">"❄"</div>
                </div>
                <div class="corner-tl"></div>
                <div class="corner-tr"></div>
                <div class="corner-bl"></div>
                <div class="corner-br"></div>

                <Nav />
                <main class="main-content">
                    <Routes>
                        <Route path="/" view=Home />
                        <Route path="/projects" view=Projects />
                        <Route path="/posts" view=PostList />
                        <Route path="/posts/:slug" view=PostView />
                        <Route path="/contacts" view=Contacts />
                    </Routes>
                </main>
                <footer class="footer">
                    <p>"Built with Leptos 🦀"</p>
                </footer>
                <XmasTree />
            </div>
        </Router>
    }
}
