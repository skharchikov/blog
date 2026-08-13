use crate::models::{Book, BookStatus};
use leptos::*;
use std::collections::BTreeMap;

/// "N book" / "N books" with correct pluralization.
fn count_label(n: usize) -> String {
    if n == 1 {
        "1 book".to_string()
    } else {
        format!("{n} books")
    }
}

/// Deterministic hue (0..360) for a book's spine, derived from its title so the
/// same book always gets the same color. FNV-1a over the bytes.
fn spine_hue(title: &str) -> u32 {
    let mut h: u32 = 2166136261;
    for b in title.bytes() {
        h ^= b as u32;
        h = h.wrapping_mul(16777619);
    }
    h % 360
}

/// Deterministic spine width (px). A little variety so the shelf doesn't look
/// like a comb; djb2 over the bytes mapped to 34..50.
fn spine_width(title: &str) -> u32 {
    let mut h: u32 = 5381;
    for b in title.bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u32);
    }
    34 + (h % 16)
}

#[component]
pub fn Bookshelf() -> impl IntoView {
    let books = Book::all_books();

    let reading: Vec<&Book> = books
        .iter()
        .filter(|b| b.status == BookStatus::Reading)
        .collect();

    // Read books grouped by the year they were finished. BTreeMap keeps years
    // ordered; we render newest-first. Books with no recorded finish date fall
    // into a trailing "undated" shelf.
    let mut by_year: BTreeMap<u32, Vec<&Book>> = BTreeMap::new();
    let mut undated: Vec<&Book> = Vec::new();
    for b in books.iter().filter(|b| b.status == BookStatus::Read) {
        match b.read_year() {
            Some(y) => by_year.entry(y).or_default().push(b),
            None => undated.push(b),
        }
    }
    let year_shelves: Vec<(u32, Vec<&Book>)> = by_year.into_iter().rev().collect();

    view! {
        <div class="bookshelf-container">
            <h1 class="page-title"><span class="prompt">"~/books $ "</span>"ls --read"</h1>

            {(!reading.is_empty()).then(|| view! {
                <section class="shelf-section">
                    <h2 class="shelf-label">"// currently reading"</h2>
                    <Shelf books=reading.clone() reading=true />
                </section>
            })}

            {year_shelves.into_iter().map(|(year, shelf)| {
                let count = shelf.len();
                view! {
                    <section class="shelf-section">
                        <h2 class="shelf-label">
                            <span class="shelf-year">{year}</span>
                            {format!(" · {}", count_label(count))}
                        </h2>
                        <Shelf books=shelf reading=false />
                    </section>
                }
            }).collect_view()}

            {(!undated.is_empty()).then(|| {
                let count = undated.len();
                view! {
                    <section class="shelf-section">
                        <h2 class="shelf-label">
                            <span class="shelf-year">"undated"</span>
                            {format!(" · {}", count_label(count))}
                        </h2>
                        <Shelf books=undated.clone() reading=false />
                    </section>
                }
            })}
        </div>
    }
}

/// A run of spines standing on a wooden shelf. Each spine reveals a cover-card
/// popover on hover / focus.
#[component]
fn Shelf(books: Vec<&'static Book>, reading: bool) -> impl IntoView {
    view! {
        <div class="shelf">
            {books.into_iter().map(|book| view! { <Spine book=book reading=reading /> }).collect_view()}
        </div>
    }
}

/// One book spine plus its hover popover (cover, title, author, rating, link).
/// A focusable div (not a button) so it can contain the Hardcover link, and so
/// keyboard focus / tap reveals the popover on devices without hover.
#[component]
fn Spine(book: &'static Book, reading: bool) -> impl IntoView {
    let hue = spine_hue(&book.title);
    let width = spine_width(&book.title);
    // Hue is --spine-hue, not --spine-h (that carries the fixed spine height).
    let style = format!("--spine-hue: {hue}; width: {width}px;");
    let spine_class = if reading { "book-spine pulled" } else { "book-spine" };
    let year = book.year.map(|y| y.to_string());
    let read_year = book.read_year().map(|y| format!("read {y}"));

    view! {
        <div class=spine_class style=style tabindex="0" aria-label=book.title.clone()>
            <span class="spine-title">{book.title.clone()}</span>

            <div class="book-pop">
                <div class="book-pop-cover">
                    {match book.cover_url.clone() {
                        Some(url) => view! {
                            <img src=url alt=format!("Cover of {}", book.title) loading="lazy" />
                        }.into_view(),
                        None => view! {
                            <div class="cover-fallback"><span>{book.title.clone()}</span></div>
                        }.into_view(),
                    }}
                </div>
                <div class="book-pop-meta">
                    <p class="book-pop-title">{book.title.clone()}</p>
                    <p class="book-pop-author">{book.authors_display()}</p>
                    <Stars rating=book.rating />
                    <p class="book-pop-facts">
                        {year.map(|y| view! { <span class="fact">{y}</span> })}
                        {read_year.map(|r| view! { <span class="fact">{r}</span> })}
                    </p>
                    <a
                        class="book-pop-link"
                        href=book.hardcover_url()
                        target="_blank"
                        rel="noopener noreferrer"
                    >"Hardcover ↗"</a>
                </div>
            </div>
        </div>
    }
}

/// Five-star rating with half-star support. Renders nothing if unrated.
#[component]
fn Stars(rating: Option<f32>) -> impl IntoView {
    rating.map(|r| {
        let stars = (1..=5).map(|n| {
            let n = n as f32;
            let class = if r >= n {
                "star full"
            } else if r >= n - 0.5 {
                "star half"
            } else {
                "star empty"
            };
            view! { <span class=class>"★"</span> }
        }).collect_view();
        view! {
            <p class="book-pop-stars" aria-label=format!("Rated {r} out of 5")>
                {stars}
                <span class="rating-num">{format!("{r:.1}")}</span>
            </p>
        }
    })
}
