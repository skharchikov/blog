use super::{posts_newest_first, reading_now, seg_view, shell_data};
use crate::models::Project;
use crate::tui::{follow, save_theme, Pane, Scene, ViewKind};
use leptos::*;
use leptos_router::*;
use tui_core::commands::{complete, run, Action, Line, Seg, WHOAMI};
use tui_core::keys::View;
use tui_core::theme::Theme;

/// One command in the transcript and what it printed.
#[derive(Clone)]
struct Entry {
    cmd: String,
    lines: Vec<Line>,
}

fn prompt() -> impl IntoView {
    view! { <span class="ps">"me@skh"</span>" "<span class="pth">"~"</span>" $ " }
}

fn line_view(line: Line) -> impl IntoView {
    view! { <div class="ln">{line.into_iter().map(seg_view).collect_view()}</div> }
}

#[component]
pub fn HomeView() -> impl IntoView {
    expect_context::<ViewKind>().0.set(View::Home);
    view! {
        <div class="body-grid cols">
            <div class="stack">
                <Scene name="home" />
                <NowPane />
            </div>
            <ShellPane />
        </div>
    }
}

/// What's current: books being read, the latest post, the first project.
#[component]
fn NowPane() -> impl IntoView {
    let reading: Vec<String> = reading_now().into_iter().map(|b| b.title.clone()).collect();
    let latest = posts_newest_first().into_iter().next();
    let project = Project::all_projects().first();
    view! {
        <Pane title="now">
            <dl class="now">
                {(!reading.is_empty()).then(move || view! {
                    <dt>"reading"</dt>
                    <dd>{reading.into_iter().map(|title| view! { <A href="/books">{title}</A> }).collect_view()}</dd>
                })}
                {latest.map(|p| view! { <dt>"latest"</dt><dd><A href=format!("/posts/{}", p.slug)>{p.title.clone()}</A></dd> })}
                {project.map(|p| view! { <dt>"project"</dt><dd><A href=format!("/projects/{}", p.slug)>{p.name.clone()}</A></dd> })}
            </dl>
        </Pane>
    }
}

#[component]
fn ShellPane() -> impl IntoView {
    let data = store_value(shell_data());
    let theme = expect_context::<RwSignal<Theme>>();
    let log = create_rw_signal(Vec::<Entry>::new());
    let input = create_rw_signal(String::new());
    let history = store_value(Vec::<String>::new());
    let cursor = store_value(0usize);
    let input_ref = create_node_ref::<html::Input>();

    // Focus the prompt on mouse/trackpad devices; on touch it would pop the keyboard.
    input_ref.on_load(|el| {
        let fine = window().match_media("(pointer: fine)").ok().flatten().map(|m| m.matches()).unwrap_or(false);
        if fine {
            let _ = el.focus();
        }
    });

    let submit = move || {
        let cmd = input.get_untracked();
        input.set(String::new());
        if cmd.trim().is_empty() {
            return;
        }
        history.update_value(|h| h.push(cmd.clone()));
        cursor.set_value(history.with_value(|h| h.len()));
        let mut lines = Vec::new();
        for action in data.with_value(|d| run(&cmd, d)) {
            match action {
                Action::Print(out) => lines.extend(out),
                Action::Navigate(path) => follow(&path),
                Action::SetTheme(t) => {
                    theme.set(t);
                    save_theme(t);
                }
                Action::Clear => {
                    log.set(Vec::new());
                    return;
                }
            }
        }
        log.update(|l| l.push(Entry { cmd, lines }));
    };

    let on_key = move |e: ev::KeyboardEvent| match e.key().as_str() {
        "Enter" => submit(),
        "Tab" => {
            e.prevent_default();
            if let Some(done) = complete(&input.get_untracked()) {
                input.set(done);
            }
        }
        "ArrowUp" | "ArrowDown" => {
            e.prevent_default();
            let len = history.with_value(|h| h.len());
            let pos = cursor.get_value();
            let next = if e.key() == "ArrowUp" { pos.saturating_sub(1) } else { (pos + 1).min(len) };
            cursor.set_value(next);
            input.set(history.with_value(|h| h.get(next).cloned().unwrap_or_default()));
        }
        "Escape" => {
            if let Some(el) = input_ref.get() {
                let _ = el.blur();
            }
        }
        _ => {}
    };

    let recent: Vec<Line> = posts_newest_first()
        .into_iter()
        .take(3)
        .map(|p| {
            vec![
                Seg::Date(p.date.clone()),
                Seg::Text("  ".into()),
                Seg::Link { label: p.title.clone(), href: format!("/posts/{}", p.slug) },
            ]
        })
        .collect();

    view! {
        <Pane title="shell" focused=true class="shell">
            <div class="ln">{prompt()}"whoami"</div>
            <div class="who">
                <b class="acc">{WHOAMI[0]}</b>
                <p>{WHOAMI[1]}</p>
                <p class="mut">{WHOAMI[2]}</p>
            </div>
            <div class="ln">{prompt()}"ls -t writing | head -3"</div>
            <div class="out">{recent.into_iter().map(line_view).collect_view()}</div>
            <For each=move || log.get().into_iter().enumerate() key=|(i, _)| *i let:entry>
                <div class="ln">{prompt()}{entry.1.cmd.clone()}</div>
                <div class="out">{entry.1.lines.clone().into_iter().map(line_view).collect_view()}</div>
            </For>
            <label class="ln input-row">
                <span class="prompt">{prompt()}</span>
                <input
                    node_ref=input_ref
                    aria-label="Shell command (type help)"
                    placeholder="type help"
                    autocomplete="off"
                    spellcheck="false"
                    prop:value=move || input.get()
                    on:input=move |e| input.set(event_target_value(&e))
                    on:keydown=on_key
                />
            </label>
        </Pane>
    }
}
