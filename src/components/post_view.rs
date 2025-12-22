use leptos::*;
use leptos_router::*;
use crate::models::BlogPost;

#[component]
pub fn PostView() -> impl IntoView {
    let params = use_params_map();
    let post = move || {
        params.with(|params| {
            params.get("slug")
                .and_then(|slug| BlogPost::find_by_slug(slug))
        })
    };

    view! {
        <div class="post-view-container">
            {move || match post() {
                Some(p) => view! {
                    <article class="post-full">
                        <header class="post-header">
                            <h1 class="post-title">{&p.title}</h1>
                            <time class="post-date">{&p.date}</time>
                            <div class="post-tags">
                                {p.tags.iter().map(|tag| {
                                    view! {
                                        <span class="tag">{tag}</span>
                                    }
                                }).collect_view()}
                            </div>
                        </header>
                        <div class="post-content" inner_html={p.content.clone()}></div>
                        <nav class="post-navigation">
                            <A href="/" class="back-link">"← Back to all posts"</A>
                        </nav>
                    </article>
                }.into_view(),
                None => view! {
                    <div class="error-container">
                        <h1>"Post Not Found"</h1>
                        <p>"The post you're looking for doesn't exist."</p>
                        <A href="/" class="back-link">"← Back to all posts"</A>
                    </div>
                }.into_view(),
            }}
        </div>
    }
}
