use super::NotFoundView;
use crate::models::Project;
use crate::tui::{Pane, Targets, ViewKind};
use leptos::*;
use leptos_router::*;
use tui_core::keys::View;

#[component]
pub fn ProjectView() -> impl IntoView {
    expect_context::<ViewKind>().0.set(View::Reader);
    let params = use_params_map();
    let project = move || params.with(|p| p.get("slug").and_then(|s| Project::find_by_slug(s)));

    Targets::bind(|| None, Some("/projects"), || None);

    move || match project() {
        Some(p) => view! {
            <Pane title=format!("projects/{}  q: back", p.slug) focused=true class="reader-pane">
                <article class="reader">
                    <p class="meta">{format!("projects/{}", p.slug)}</p>
                    <h1>{p.name.clone()}</h1>
                    <p class="summary">{p.description.clone()}</p>
                    <p class="tags">
                        {p.tags.iter().map(|t| view! { <span>{format!("#{t}")}</span> }).collect_view()}
                        <a class="gh" href=p.github_url.clone() target="_blank" rel="noopener noreferrer">"view on GitHub ↗"</a>
                    </p>
                    <div class="post-content project-content" inner_html=p.content.clone()></div>
                    <p class="back"><A href="/projects">"← all projects"</A></p>
                </article>
            </Pane>
        }
        .into_view(),
        None => view! { <NotFoundView /> }.into_view(),
    }
}
