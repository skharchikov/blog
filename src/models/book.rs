use std::sync::OnceLock;

/// Reading status, mirroring Hardcover's `status_id`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BookStatus {
    Want,
    Reading,
    Read,
    Paused,
    Dnf,
}

impl BookStatus {
    /// Map Hardcover's numeric `status_id` (1=want, 2=reading, 3=read,
    /// 4=paused, 5=DNF). Unknown ids fall back to `Read`.
    pub fn from_id(id: u32) -> Self {
        match id {
            1 => BookStatus::Want,
            2 => BookStatus::Reading,
            4 => BookStatus::Paused,
            5 => BookStatus::Dnf,
            _ => BookStatus::Read,
        }
    }
}

/// One book on the shelf. Optional fields are `None` when Hardcover has no data
/// (e.g. no cover art, unrated, unknown release year, no recorded read date).
#[derive(Clone, Debug, PartialEq)]
pub struct Book {
    pub title: String,
    pub authors: Vec<String>,
    pub cover_url: Option<String>,
    pub rating: Option<f32>,
    pub status: BookStatus,
    pub year: Option<u32>,
    /// Finish date, `YYYY-MM-DD`, from the most recent read.
    pub read_date: Option<String>,
    pub slug: String,
}

static BOOKS: OnceLock<Vec<Book>> = OnceLock::new();

impl Book {
    pub fn all_books() -> &'static Vec<Self> {
        BOOKS.get_or_init(|| include!(concat!(env!("OUT_DIR"), "/generated_books.rs")))
    }

    /// Link back to the book's page on Hardcover.
    pub fn hardcover_url(&self) -> String {
        format!("https://hardcover.app/books/{}", self.slug)
    }

    /// Year the book was finished, parsed from `read_date`.
    pub fn read_year(&self) -> Option<u32> {
        self.read_date
            .as_ref()
            .and_then(|d| d.get(0..4))
            .and_then(|y| y.parse().ok())
    }

    /// Authors joined for display, e.g. "Steve Klabnik & Carol Nichols".
    pub fn authors_display(&self) -> String {
        match self.authors.as_slice() {
            [] => "Unknown".to_string(),
            [a] => a.clone(),
            [head @ .., last] => format!("{} & {}", head.join(", "), last),
        }
    }
}
