//! One view per tab, plus the shared pieces they use.

mod home;
mod post;
mod writing;

pub use home::HomeView;
pub use post::PostView;
pub use writing::WritingView;

use crate::models::{BlogPost, Book, BookStatus, Project};
use leptos::*;
use leptos_router::*;
use tui_core::commands::{PostRef, Seg, ShellData};

pub const EMAILS: [&str; 2] = ["me@skh.rs", "contact@skharchikov.com"];

/// Posts newest first (dates are `YYYY-MM-DD`, so a string compare sorts).
pub fn posts_newest_first() -> Vec<&'static BlogPost> {
    let mut posts: Vec<_> = BlogPost::all_posts().iter().collect();
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

pub fn reading_now() -> Vec<&'static Book> {
    Book::all_books().iter().filter(|b| b.status == BookStatus::Reading).collect()
}

/// What the home shell knows about the site.
pub fn shell_data() -> ShellData {
    ShellData {
        posts: posts_newest_first()
            .into_iter()
            .map(|p| PostRef { date: p.date.clone(), title: p.title.clone(), slug: p.slug.clone() })
            .collect(),
        projects: Project::all_projects().iter().map(|p| p.slug.clone()).collect(),
        reading: reading_now().into_iter().map(|b| b.title.clone()).collect(),
        emails: EMAILS,
    }
}

/// One piece of shell output. Site paths go through the router; other links
/// (mailto) are plain anchors.
pub fn seg_view(seg: Seg) -> View {
    match seg {
        Seg::Text(t) => view! { <span>{t}</span> }.into_view(),
        Seg::Accent(t) => view! { <b class="acc">{t}</b> }.into_view(),
        Seg::Date(t) => view! { <span class="date">{t}</span> }.into_view(),
        Seg::Link { label, href } if href.starts_with('/') => view! { <A href=href>{label}</A> }.into_view(),
        Seg::Link { label, href } => view! { <a href=href>{label}</a> }.into_view(),
    }
}
