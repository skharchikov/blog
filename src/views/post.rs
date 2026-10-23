use super::writing::read_minutes;
use crate::components::{Giscus, ViewCounter};
use super::NotFoundView;
use crate::models::BlogPost;
use crate::tui::{Pane, Targets, ViewKind};
use leptos::*;
use leptos_router::*;
use tui_core::keys::View;
use tui_core::theme::Theme;

#[component]
pub fn PostView() -> impl IntoView {
    expect_context::<ViewKind>().0.set(View::Reader);
    let theme = expect_context::<RwSignal<Theme>>();
    let params = use_params_map();
    let post = move || params.with(|p| p.get("slug").and_then(|s| BlogPost::find_by_slug(s)));

    Targets::bind(|| None, Some("/posts"), || None);

    move || match post() {
        Some(p) => view! {
            <Pane title=format!("writing/{}.md  q: back", p.slug) focused=true class="reader-pane">
                <article class="reader">
                    <p class="meta">
                        <time>{p.date.clone()}</time>
                        {format!(" · {} min read · ", read_minutes(p))}
                        <ViewCounter path=format!("/posts/{}", p.slug) />
                    </p>
                    <h1>{p.title.clone()}</h1>
                    <p class="tags">
                        {p.tags.iter().map(|t| view! { <A href=format!("/posts?tag={t}")>{format!("#{t}")}</A> }).collect_view()}
                    </p>
                    <div class="post-content" inner_html=p.content.clone()></div>
                    <Giscus light=Signal::derive(move || theme.get().is_light()) />
                    <p class="back"><A href="/posts">"← all posts"</A></p>
                </article>
            </Pane>
        }
        .into_view(),
        None => view! { <NotFoundView /> }.into_view(),
    }
}
