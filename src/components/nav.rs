use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="nav-content">
                <A href="/" class="nav-brand">
                    <h1>"My Rust Blog"</h1>
                </A>
            </div>
        </nav>
    }
}
