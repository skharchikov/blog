use crate::models::{Book, BookStatus};
use crate::tui::{focus_follows, on_intent, reveal, Pane, Scene, Selections, Targets, ViewKind};
use leptos::*;
use tui_core::keys::{Intent, View};
use tui_core::shelf::{group, GroupLabel};
use wasm_bindgen::JsCast;

/// The cover visually above (`dy < 0`) or below the cover at `from`: the
/// nearest row in that direction, then the closest cover by horizontal centre.
/// The wall is several grids (one per year), so this works on layout, not on
/// a column count.
fn vertical_neighbour(wall: &web_sys::HtmlElement, from: usize, dy: i32) -> Option<usize> {
    let covers = wall.query_selector_all(".cv").ok()?;
    let boxes: Vec<(i32, i32)> = (0..covers.length())
        .filter_map(|i| covers.item(i)?.dyn_into::<web_sys::HtmlElement>().ok())
        .map(|el| (el.offset_top(), el.offset_left() + el.offset_width() / 2))
        .collect();
    let (top, centre) = *boxes.get(from)?;
    let row = boxes
        .iter()
        .map(|(t, _)| *t)
        .filter(|t| if dy > 0 { *t > top + 4 } else { *t < top - 4 })
        .reduce(|a, b| if dy > 0 { a.min(b) } else { a.max(b) })?;
    boxes
        .iter()
        .enumerate()
        .filter(|(_, (t, _))| (t - row).abs() <= 4)
        .min_by_key(|(_, (_, c))| (c - centre).abs())
        .map(|(i, _)| i)
}

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
    let selected = expect_context::<Selections>().books;
    if selected.get_untracked() >= total {
        selected.set(0);
    }
    let current = move || order.with_value(|o| o.get(selected.get()).copied());

    Targets::bind(move || current().map(Book::hardcover_url), || None, || None);

    let wall = create_node_ref::<html::Div>();
    // Keep the selected cover visible when it moves (not on first render).
    create_effect(move |first: Option<()>| {
        let i = selected.get();
        if first.is_some() {
            let cover = wall.get_untracked().and_then(|w| w.query_selector_all(".cv").ok()).and_then(|c| c.item(i as u32));
            if let Some(el) = cover.and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                reveal(&el);
            }
        }
    });
    on_intent(move |i| {
        match i {
            Intent::MoveGrid { dx, dy: 0 } => {
                let next = selected.get_untracked() as i32 + dx;
                selected.set(next.clamp(0, total.saturating_sub(1) as i32) as usize);
            }
            Intent::MoveGrid { dy, .. } => {
                if let Some(next) = wall.get_untracked().and_then(|w| vertical_neighbour(&w, selected.get_untracked(), dy)) {
                    selected.set(next);
                }
            }
            _ => return,
        }
        focus_follows(".wall .cv", selected.get_untracked());
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
                                // A plain click shows the book in the detail pane; a click on the
                                // selected cover (or any modifier click) opens Hardcover.
                                let on_click = move |e: ev::MouseEvent| {
                                    let modified = e.meta_key() || e.ctrl_key() || e.shift_key() || e.alt_key();
                                    if !modified && selected.get_untracked() != pos {
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
                                        aria-current=move || (selected.get() == pos).then_some("true")
                                        on:click=on_click
                                        // Keyboard focus selects. A mouse click also focuses the
                                        // link (before on:click), but that must not count as
                                        // already selected, so only :focus-visible does.
                                        on:focus=move |e: ev::FocusEvent| {
                                            let keyboard = event_target::<web_sys::Element>(&e).matches(":focus-visible").unwrap_or(true);
                                            if keyboard {
                                                selected.set(pos);
                                            }
                                        }
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
