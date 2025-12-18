use leptos::*;
use leptos_router::*;
use crate::models::BlogPost;

#[component]
pub fn PostList() -> impl IntoView {
    let posts = BlogPost::all_posts();

    view! {
        <div class="post-list-container">
            <h1 class="page-title">"Recent Posts"</h1>
            <div class="post-grid">
                {posts.into_iter().map(|post| {
                    view! {
                        <article class="post-card">
                            <div class="post-card-header">
                                <h2>
                                    <A href={format!("/post/{}", post.slug)} class="post-title-link">
                                        {&post.title}
                                    </A>
                                </h2>
                                <time class="post-date">{&post.date}</time>
                            </div>
                            <p class="post-excerpt">{&post.excerpt}</p>
                            <div class="post-tags">
                                {post.tags.iter().map(|tag| {
                                    view! {
                                        <span class="tag">{tag}</span>
                                    }
                                }).collect_view()}
                            </div>
                            <A href={format!("/post/{}", post.slug)} class="read-more">
                                "Read more →"
                            </A>
                        </article>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
