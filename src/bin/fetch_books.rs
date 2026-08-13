//! Refresh `books/library.json` from the Hardcover GraphQL API.
//!
//! Runs on the host only (never wasm), behind the `fetch-books` feature:
//!
//! ```sh
//! cargo run --bin fetch_books --features fetch-books \
//!     --target "$(rustc -vV | sed -n 's/^host: //p')"
//! ```
//!
//! Reads the API token from `HARDCOVER_TOKEN` (via a `.env` file if present).
//! The token is a personal read/write credential and MUST stay server-side —
//! it is never shipped to the browser. `build.rs` reads the JSON this writes
//! and codegens a static `Vec<Book>`, so the SPA itself makes no API calls.
//!
//! Fetches the "currently reading" (status_id 2) and "read" (status_id 3)
//! shelves, paginated, and writes them sorted (reading first, then read newest
//! first) to `books/library.json`.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

const ENDPOINT: &str = "https://api.hardcover.app/v1/graphql";
const PAGE_SIZE: i64 = 100;

/// GraphQL query. `me` returns an array (take the first element).
const QUERY: &str = r#"
query Books($limit: Int!, $offset: Int!) {
  me {
    user_books(
      where: { status_id: { _in: [2, 3] } },
      order_by: { updated_at: desc },
      limit: $limit,
      offset: $offset
    ) {
      rating
      status_id
      book {
        title
        release_year
        slug
        image { url }
        contributions { contribution author { name } }
      }
      user_book_reads { finished_at }
    }
  }
}
"#;

// ---- API response shapes -------------------------------------------------

#[derive(Deserialize)]
struct GraphQlResponse {
    data: Option<Data>,
    #[serde(default)]
    errors: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct Data {
    me: Vec<Me>,
}

#[derive(Deserialize)]
struct Me {
    user_books: Vec<UserBook>,
}

#[derive(Deserialize)]
struct UserBook {
    rating: Option<f32>,
    status_id: u32,
    book: Option<ApiBook>,
    #[serde(default)]
    user_book_reads: Vec<ReadEntry>,
}

#[derive(Deserialize)]
struct ApiBook {
    title: String,
    release_year: Option<u32>,
    slug: Option<String>,
    image: Option<Image>,
    #[serde(default)]
    contributions: Vec<Contribution>,
}

#[derive(Deserialize)]
struct Image {
    url: Option<String>,
}

#[derive(Deserialize)]
struct Contribution {
    /// Role string, e.g. "Translator" / "Editor"; null for a primary author.
    contribution: Option<String>,
    author: Option<Author>,
}

#[derive(Deserialize)]
struct Author {
    name: Option<String>,
}

#[derive(Deserialize)]
struct ReadEntry {
    finished_at: Option<String>,
}

// Writer side of the library.json format; reader is the identical BookRecord
// in build.rs. Keep fields in sync — a mismatch silently drops data to None.
#[derive(Serialize)]
struct BookRecord {
    title: String,
    authors: Vec<String>,
    cover_url: Option<String>,
    rating: Option<f32>,
    status_id: u32,
    year: Option<u32>,
    read_date: Option<String>,
    slug: String,
}

#[derive(Serialize)]
struct Library {
    books: Vec<BookRecord>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load .env if present; ignore if absent (CI passes the var directly).
    let _ = dotenvy::dotenv();
    let token = std::env::var("HARDCOVER_TOKEN")
        .map_err(|_| "HARDCOVER_TOKEN not set (add it to .env or the environment)")?;

    let client = reqwest::blocking::Client::new();

    let mut records: Vec<BookRecord> = Vec::new();
    let mut offset: i64 = 0;
    loop {
        let page = fetch_page(&client, &token, offset)?;
        let count = page.len();
        for ub in page {
            if let Some(rec) = to_record(ub) {
                records.push(rec);
            }
        }
        if (count as i64) < PAGE_SIZE {
            break;
        }
        offset += PAGE_SIZE;
    }

    // Reading first, then read; within each, newest read date first
    // (books without a date sort last).
    records.sort_by(|a, b| {
        let rank = |s: u32| if s == 2 { 0 } else { 1 };
        rank(a.status_id)
            .cmp(&rank(b.status_id))
            .then_with(|| b.read_date.cmp(&a.read_date))
    });

    let library = Library { books: records };
    let json = serde_json::to_string_pretty(&library)?;

    fs::create_dir_all("books")?;
    let out = Path::new("books/library.json");
    fs::write(out, json + "\n")?;

    println!(
        "fetch_books: wrote {} book(s) to {}",
        library.books.len(),
        out.display()
    );
    Ok(())
}

/// Fetch one page of `user_books` at the given offset.
fn fetch_page(
    client: &reqwest::blocking::Client,
    token: &str,
    offset: i64,
) -> Result<Vec<UserBook>, Box<dyn Error>> {
    let body = serde_json::json!({
        "query": QUERY,
        "variables": { "limit": PAGE_SIZE, "offset": offset },
    });

    let resp: GraphQlResponse = client
        .post(ENDPOINT)
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .json(&body)
        .send()?
        .error_for_status()?
        .json()?;

    if let Some(errors) = resp.errors {
        return Err(format!("Hardcover GraphQL error: {errors}").into());
    }

    let data = resp.data.ok_or("Hardcover response had no data")?;
    let me = data.me.into_iter().next().ok_or("`me` was empty")?;
    Ok(me.user_books)
}

/// Convert an API `user_book` into an output record, dropping entries with no
/// usable book payload or slug (needed for the Hardcover link + build codegen).
fn to_record(ub: UserBook) -> Option<BookRecord> {
    let book = ub.book?;
    let slug = book.slug?;

    // Keep only primary authors (null role); translators/editors carry a role
    // string. If a book somehow lists no primary author, fall back to all
    // contributors so the entry isn't left author-less.
    let mut authors: Vec<String> = Vec::new();
    let mut fallback: Vec<String> = Vec::new();
    for c in book.contributions {
        let is_primary = c.contribution.is_none();
        if let Some(name) = c.author.and_then(|a| a.name) {
            let name = name.split_whitespace().collect::<Vec<_>>().join(" ");
            if name.is_empty() {
                continue;
            }
            if is_primary && !authors.contains(&name) {
                authors.push(name.clone());
            }
            if !fallback.contains(&name) {
                fallback.push(name);
            }
        }
    }
    if authors.is_empty() {
        authors = fallback;
    }

    // Most recent finish date across all recorded reads.
    let read_date = ub
        .user_book_reads
        .into_iter()
        .filter_map(|r| r.finished_at)
        .max();

    Some(BookRecord {
        title: book.title,
        authors,
        cover_url: book.image.and_then(|i| i.url),
        rating: ub.rating,
        status_id: ub.status_id,
        year: book.release_year,
        read_date,
        slug,
    })
}
