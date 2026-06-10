use crate::models::BlogPost;
use leptos::*;
use leptos_router::*;

/// Estimate read time in minutes from raw post content (~200 wpm, min 1).
fn read_minutes(content: &str) -> usize {
    let words = content.split_whitespace().count();
    std::cmp::max(1, (words + 199) / 200)
}

#[component]
pub fn PostList() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    let posts = BlogPost::all_posts();

    // Selected tag is derived from the URL (?tag=...), not local state.
    let selected_tag = move || query.with(|q| q.get("tag").cloned());

    let filtered_posts = move || {
        if let Some(tag) = selected_tag() {
            posts
                .iter()
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

    // Count of posts carrying a given tag (for the "(n)" badge).
    let count_for = move |tag: &str| posts.iter().filter(|p| p.tags.contains(&tag.to_string())).count();

    // Navigate helpers: "All" clears the query, a tag sets ?tag=...
    let nav_all = {
        let navigate = navigate.clone();
        move || navigate("/posts", Default::default())
    };
    let nav_tag = {
        let navigate = navigate.clone();
        move |tag: &str| navigate(&format!("/posts?tag={}", tag), Default::default())
    };

    // Pre-clone nav_tag for the tag-filter section (the post-grid section owns the original).
    let nav_tag_filter = nav_tag.clone();

    view! {
        <div class="post-list-container">
            <h1 class="page-title"><span class="prompt">"~/posts $ "</span>"ls"</h1>

            <div class="tag-filter">
                <button
                    class="tag-filter-btn"
                    class:active={move || selected_tag().is_none()}
                    attr:aria-pressed={move || if selected_tag().is_none() { "true" } else { "false" }}
                    on:click={
                        let nav_all = nav_all.clone();
                        move |_| nav_all()
                    }
                >
                    "all"
                </button>
                {move || {
                    let nav_tag = nav_tag_filter.clone();
                    all_tags().into_iter().map(move |tag| {
                        let tag_active = tag.clone();
                        let tag_click = tag.clone();
                        let tag_label = tag.clone();
                        let n = count_for(&tag);
                        let nav_tag = nav_tag.clone();
                        view! {
                            <button
                                class="tag-filter-btn"
                                class:active={move || selected_tag().as_deref() == Some(&tag_active)}
                                attr:aria-pressed={
                                    let t = tag_active.clone();
                                    move || if selected_tag().as_deref() == Some(&t) { "true" } else { "false" }
                                }
                                on:click=move |_| nav_tag(&tag_click)
                            >
                                {format!("{} ({})", tag_label, n)}
                            </button>
                        }
                    }).collect_view()
                }}
            </div>

            <div class="post-grid">
                {move || {
                    let items = filtered_posts();
                    if items.is_empty() {
                        view! { <p class="post-empty">"No posts with this tag yet."</p> }.into_view()
                    } else {
                        items.into_iter().map(|post| {
                            let nav_tag = nav_tag.clone();
                            let minutes = read_minutes(&post.content);
                            view! {
                                <A href={format!("/posts/{}", post.slug)} class="post-card-link">
                                    <article class="post-card">
                                        <div class="post-card-header">
                                            <h2 class="post-card-title">{&post.title}</h2>
                                            <time class="post-date">{format!("{} · {} min", post.date, minutes)}</time>
                                        </div>
                                        <p class="post-excerpt">{&post.excerpt}</p>
                                        <div class="post-tags">
                                            {post.tags.iter().map(|tag| {
                                                let tag_click = tag.clone();
                                                let nav_tag = nav_tag.clone();
                                                view! {
                                                    <span
                                                        class="tag"
                                                        on:click=move |ev| {
                                                            ev.prevent_default();
                                                            ev.stop_propagation();
                                                            nav_tag(&tag_click);
                                                        }
                                                    >
                                                        {tag}
                                                    </span>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </article>
                                </A>
                            }
                        }).collect_view().into_view()
                    }
                }}
            </div>
        </div>
    }
}
