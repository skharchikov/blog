use leptos::*;
use leptos_router::*;
use crate::models::BlogPost;

#[component]
pub fn PostList() -> impl IntoView {
    let (selected_tag, set_selected_tag) = create_signal(Option::<String>::None);

    let posts = BlogPost::all_posts();

    let filtered_posts = move || {
        if let Some(tag) = selected_tag.get() {
            posts.iter()
                .filter(|post| post.tags.contains(&tag))
                .cloned()
                .collect::<Vec<_>>()
        } else {
            posts.to_vec()
        }
    };

    let all_tags = move || {
        let mut tags = std::collections::HashSet::new();
        for post in posts.iter() {
            for tag in &post.tags {
                tags.insert(tag.clone());
            }
        }
        let mut tag_vec: Vec<_> = tags.into_iter().collect();
        tag_vec.sort();
        tag_vec
    };

    view! {
        <div class="post-list-container">
            <h1 class="page-title">"Posts"</h1>

            <div class="tag-filter">
                <button
                    class="tag-filter-btn"
                    class:active={move || selected_tag.get().is_none()}
                    on:click=move |_| set_selected_tag.set(None)
                >
                    "All"
                </button>
                {all_tags().into_iter().map(|tag| {
                    let tag_for_active = tag.clone();
                    let tag_for_click = tag.clone();
                    let tag_for_display = tag.clone();
                    view! {
                        <button
                            class="tag-filter-btn"
                            class:active={move || selected_tag.get().as_ref() == Some(&tag_for_active)}
                            on:click=move |_| set_selected_tag.set(Some(tag_for_click.clone()))
                        >
                            {tag_for_display}
                        </button>
                    }
                }).collect_view()}
            </div>

            <div class="post-grid">
                {move || filtered_posts().into_iter().map(|post| {
                    view! {
                        <article class="post-card">
                            <div class="post-card-header">
                                <A href={format!("/posts/{}", post.slug)} class="post-title-link">
                                    <h2>{&post.title}</h2>
                                </A>
                                <time class="post-date">{&post.date}</time>
                            </div>
                            <p class="post-excerpt">{&post.excerpt}</p>
                            <div class="post-tags">
                                {post.tags.iter().map(|tag| {
                                    let tag_clone = tag.clone();
                                    view! {
                                        <span
                                            class="tag"
                                            on:click=move |_| set_selected_tag.set(Some(tag_clone.clone()))
                                        >
                                            {tag}
                                        </span>
                                    }
                                }).collect_view()}
                            </div>
                        </article>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
