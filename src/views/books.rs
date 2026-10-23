use crate::models::{Book, BookStatus};
use crate::tui::{on_intent, Pane, Scene, Targets, ViewKind};
use leptos::*;
use tui_core::keys::{Intent, View};
use tui_core::shelf::{group, GroupLabel};
use wasm_bindgen::JsCast;

/// Cover width plus grid gap (px), used to count columns for up/down moves.
const COVER_STEP: i32 = 72;

fn stars(rating: Option<f32>) -> String {
    match rating {
        Some(r) => {
            let full = r.round().clamp(0.0, 5.0) as usize;
            format!("{}{} {r:.1}", "★".repeat(full), "☆".repeat(5 - full))
        }
        None => "not rated".to_string(),
    }
}

fn cover(book: &Book, class: &'static str) -> leptos::View {
    match &book.cover_url {
        Some(url) => view! { <img class=class src=url.clone() alt="" loading="lazy" /> }.into_view(),
        None => view! { <span class=format!("{class} fallback")>{book.title.clone()}</span> }.into_view(),
    }
}

#[component]
pub fn BooksView() -> impl IntoView {
    expect_context::<ViewKind>().0.set(View::Grid);
    let shelf: Vec<&'static Book> = Book::all_books()
        .iter()
        .filter(|b| matches!(b.status, BookStatus::Reading | BookStatus::Read))
        .collect();
    let groups = group(&shelf.iter().map(|b| (b.status == BookStatus::Reading, b.read_year())).collect::<Vec<_>>());
    // Wall order = group order; `order[k]` is the k-th cover on the wall.
    let order: Vec<&'static Book> = groups.iter().flat_map(|g| g.indices.iter().map(|&i| shelf[i])).collect();
    let total = order.len();
    let order = store_value(order);
    let selected = create_rw_signal(0usize);
    let current = move || order.with_value(|o| o.get(selected.get()).copied());

    Targets::bind(move || current().map(Book::hardcover_url), None, || None);

    let wall = create_node_ref::<html::Div>();
    on_intent(move |i| {
        if let Intent::MoveGrid { dx, dy } = i {
            let cols = wall
                .get_untracked()
                .and_then(|w| w.query_selector(".covers").ok().flatten())
                .and_then(|g| g.dyn_into::<web_sys::HtmlElement>().ok())
                .map_or(1, |g| (g.client_width() / COVER_STEP).max(1));
            let next = selected.get_untracked() as i32 + dx + dy * cols;
            selected.set(next.clamp(0, total.saturating_sub(1) as i32) as usize);
        }
    });

    let detail = move || {
        current().map(|b| {
            let status = match (b.status, b.read_year()) {
                (BookStatus::Reading, _) => "reading now".to_string(),
                (_, Some(y)) => format!("read {y}"),
                _ => "read (undated)".to_string(),
            };
            view! {
                <div class="book-detail">
                    {cover(b, "big")}
                    <h2>{b.title.clone()}</h2>
                    <p class="mut">{b.authors_display()}</p>
                    <p class="stars">{stars(b.rating)}</p>
                    <p class="mut">{status}{b.year.map(|y| format!(" · published {y}"))}</p>
                    <p><a href=b.hardcover_url() target="_blank" rel="noopener noreferrer">"hardcover ↗"</a></p>
                </div>
            }
        })
    };

    let mut start = 0usize;
    let sections = groups
        .iter()
        .map(|g| {
            let heading = match g.label {
                GroupLabel::ReadingNow => "reading now".to_string(),
                GroupLabel::Year(y) => format!("{y} · {}", g.indices.len()),
                GroupLabel::Earlier => format!("earlier · {}", g.indices.len()),
            };
            let first = start;
            start += g.indices.len();
            let reading = g.label == GroupLabel::ReadingNow;
            view! {
                <section class="shelf">
                    <h3>{heading}</h3>
                    <div class="covers">
                        {g.indices
                            .iter()
                            .enumerate()
                            .map(|(k, &i)| {
                                let pos = first + k;
                                let book = shelf[i];
                                // first click selects, a click on the selected cover opens Hardcover
                                let on_click = move |e: ev::MouseEvent| {
                                    if selected.get_untracked() != pos {
                                        e.prevent_default();
                                        selected.set(pos);
                                    }
                                };
                                view! {
                                    <a
                                        class="cv"
                                        class:sel=move || selected.get() == pos
                                        class:reading=reading
                                        href=book.hardcover_url()
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        title=book.title.clone()
                                        aria-label=format!("{} by {}", book.title, book.authors_display())
                                        on:click=on_click
                                    >
                                        {cover(book, "img")}
                                    </a>
                                }
                            })
                            .collect_view()}
                    </div>
                </section>
            }
        })
        .collect_view();

    view! {
        <div class="body-grid cols books">
            <div class="stack">
                <Scene name="books" />
                <Pane title="book">{detail}</Pane>
            </div>
            <Pane title=format!("books/ · {total}") focused=true>
                <div class="wall" node_ref=wall>{sections}</div>
            </Pane>
        </div>
    }
}
