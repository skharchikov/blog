use crate::models::Project;
use crate::tui::{focus_follows, on_intent, ListRow, Pane, Scene, SelectList, Selections, Targets, ViewKind};
use leptos::*;
use tui_core::keys::{Intent, View};
use tui_core::readout;

#[component]
pub fn ProjectsView() -> impl IntoView {
    expect_context::<ViewKind>().0.set(View::List);
    let projects = Project::all_projects();

    let stats = {
        let names: Vec<&str> = projects.iter().map(|p| p.slug.as_str()).collect();
        let tags: Vec<Vec<&str>> = projects.iter().map(|p| p.tags.iter().map(String::as_str).collect()).collect();
        let tag_refs: Vec<&[&str]> = tags.iter().map(Vec::as_slice).collect();
        readout::projects(&names, &tag_refs)
    };

    let rows: Vec<ListRow> = projects
        .iter()
        .map(|p| ListRow {
            left: String::new(),
            title: format!("{}/", p.slug),
            right: p.tags.iter().map(|t| format!("#{t}")).collect::<Vec<_>>().join(" "),
            href: format!("/projects/{}", p.slug),
        })
        .collect();
    let selections = expect_context::<Selections>();
    let selected = selections.projects;
    if selected.get_untracked() >= projects.len() {
        selected.set(0);
    }
    selections.select_opened(selected, &rows.iter().map(|r| r.href.clone()).collect::<Vec<_>>());
    let rows = Signal::derive(move || rows.clone());
    let current = move || projects.get(selected.get());

    Targets::bind(
        move || current().map(|p| format!("/projects/{}", p.slug)),
        || None,
        move || current().map(|p| p.github_url.clone()),
    );
    on_intent(move |i| {
        if let Intent::Move(d) = i {
            let last = projects.len().saturating_sub(1) as i32;
            selected.update(|s| *s = (*s as i32 + d).clamp(0, last) as usize);
            focus_follows(".list a.row", selected.get_untracked());
        }
    });

    let preview = move || {
        current().map(|p| {
            view! {
                <p class="meta">{format!("projects/{}", p.slug)}</p>
                <h2 class="prev-title">{p.name.clone()}</h2>
                <p class="prev-text">{p.description.clone()}</p>
                <p class="prev-links">
                    <a href=p.github_url.clone() target="_blank" rel="noopener noreferrer">"github ↗"</a>
                </p>
            }
        })
    };

    view! {
        <div class="body-grid cols projects">
            <Scene name="projects" readout=stats />
            <Pane title="preview" class="preview">{preview}</Pane>
            <Pane title="ls projects/" focused=true class="wide">
                <SelectList items=rows selected=selected />
            </Pane>
        </div>
    }
}
