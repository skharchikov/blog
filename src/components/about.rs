use leptos::*;
use leptos_router::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="about-container">
            <article class="about-content">
                <header class="about-header">
                    <h1>"About Me"</h1>
                </header>
                <div class="about-body">
                    <p>
                        "Hi! I'm a Rust enthusiast and web developer passionate about building fast,
                        reliable, and elegant web applications."
                    </p>
                    <p>
                        "This blog is built with Leptos, a modern Rust web framework that compiles to
                        WebAssembly. I write about Rust, web development, and software engineering topics
                        that interest me."
                    </p>
                    <h2>"What I Do"</h2>
                    <ul>
                        <li>"Build web applications with Rust and WebAssembly"</li>
                        <li>"Explore modern web frameworks and technologies"</li>
                        <li>"Share knowledge through blog posts and tutorials"</li>
                    </ul>
                    <h2>"Get in Touch"</h2>
                    <p>
                        "Feel free to reach out if you'd like to discuss Rust, web development,
                        or just want to say hello!"
                    </p>
                </div>
                <nav class="about-navigation">
                    <A href="/" class="back-link">"← Back to posts"</A>
                </nav>
            </article>
        </div>
    }
}
