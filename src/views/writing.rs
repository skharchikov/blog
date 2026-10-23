use super::posts_newest_first;
use crate::models::BlogPost;
use crate::tui::{on_intent, ListRow, Pane, Scene, SelectList, Selections, Targets, ViewKind};
use leptos::*;
use leptos_router::*;
use tui_core::keys::{Intent, View};
use tui_core::readout;
use wasm_bindgen::JsCast;

/// Read time in minutes: the post's explicit override, or a ~200 wpm estimate.
/// `content` is generated HTML, so only visible text is counted (tags skipped),
/// otherwise tag/attribute names would inflate the estimate.
pub fn read_minutes(post: &BlogPost) -> u32 {
    post.read_time.unwrap_or_else(|| {
        let mut in_tag = false;
        let mut in_word = false;
        let mut words = 0usize;
        for c in post.content.chars() {
            match c {
                '<' => {
                    in_tag = true;
                    in_word = false;
                }
                '>' => in_tag = false,
                _ if in_tag => {}
                _ if c.is_whitespace() => in_word = false,
                _ => {
                    if !in_word {
                        words += 1;
                        in_word = true;
                    }
                }
            }
        }
        std::cmp::max(1, (words + 199) / 200) as u32
    })
}

#[component]
pub fn WritingView() -> impl IntoView {
    expect_context::<ViewKind>().0.set(View::List);
    let query = use_query_map();
    let tag = move || query.with(|q| q.get("tag").cloned());
    let all = posts_newest_first();

    let mut tags: Vec<String> = all.iter().flat_map(|p| p.tags.clone()).collect();
    tags.sort();
    tags.dedup();

    let stats = {
        let posts: Vec<(&str, Vec<&str>)> =
            all.iter().map(|p| (p.date.as_str(), p.tags.iter().map(String::as_str).collect())).collect();
        let refs: Vec<(&str, &[&str])> = posts.iter().map(|(d, t)| (*d, t.as_slice())).collect();
        readout::writing(&refs)
    };

    let posts = Signal::derive(move || {
        let tag = tag();
        posts_newest_first()
            .into_iter()
            .filter(|p| tag.as_ref().map_or(true, |t| p.tags.contains(t)))
            .collect::<Vec<_>>()
    });
    let rows = Signal::derive(move || {
        posts
            .get()
            .into_iter()
            .map(|p| ListRow {
                left: p.date.clone(),
                title: p.title.clone(),
                right: p.tags.iter().map(|t| format!("#{t}")).collect::<Vec<_>>().join(" "),
                href: format!("/posts/{}", p.slug),
            })
            .collect::<Vec<_>>()
    });
    // Shared with the shell, so the place in the list survives opening a post.
    let selected = expect_context::<Selections>().writing;
    // A changed filter starts at the top of the new list; a remembered
    // position is clamped to the current length.
    create_effect(move |prev: Option<Option<String>>| {
        let t = tag();
        let len = rows.with(Vec::len);
        if prev.is_some_and(|p| p != t) || selected.get_untracked() >= len {
            selected.set(0);
        }
        t
    });

    Targets::bind(move || rows.with(|r| r.get(selected.get()).map(|row| row.href.clone())), None, || None);

    let chips = create_node_ref::<html::Div>();
    on_intent(move |i| {
        let len = rows.with_untracked(Vec::len);
        match i {
            Intent::Move(d) if len > 0 => {
                selected.update(|s| *s = (*s as i32 + d).clamp(0, len as i32 - 1) as usize)
            }
            Intent::FocusFilter => {
                let first = chips.get_untracked().and_then(|c| c.query_selector("a").ok().flatten());
                if let Some(el) = first.and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok()) {
                    let _ = el.focus();
                }
            }
            _ => {}
        }
    });

    let preview = move || {
        posts.get().get(selected.get()).copied().map(|p| {
            view! {
                <p class="meta">{format!("{} · {} min read", p.date, read_minutes(p))}</p>
                <h2 class="prev-title">{p.title.clone()}</h2>
                <p class="prev-text">{p.excerpt.clone()}</p>
            }
        })
    };

    view! {
        <div class="body-grid cols writing">
            <Scene name="writing" readout=stats />
            <Pane title="preview" class="preview">{preview}</Pane>
            <Pane title="ls -l writing/" focused=true class="wide">
                <div class="chips" node_ref=chips aria-label="Filter by tag">
                    <A href="/posts" class="chip" exact=true>"all"</A>
                    {tags
                        .into_iter()
                        .map(|t| {
                            let href = format!("/posts?tag={t}");
                            let on = {
                                let t = t.clone();
                                move || tag().as_deref() == Some(t.as_str())
                            };
                            view! { <a href=href class="chip" class:on=on>{format!("#{t}")}</a> }
                        })
                        .collect_view()}
                </div>
                <SelectList items=rows selected=selected />
                {move || rows.with(Vec::is_empty).then(|| view! { <p class="mut">"No posts with this tag yet."</p> })}
            </Pane>
        </div>
    }
}
