//! Static-HTML prerenderer for social/link previews.
//!
//! The site is a client-side-rendered (CSR) Leptos SPA: every route is served
//! the same `index.html`, and the router swaps views in the browser. Social
//! crawlers (Twitter/X, Facebook, LinkedIn, Slack, ...) do not run wasm, so they
//! only ever see the default `<head>` meta tags — every shared link shows the
//! site title instead of the post's own description.
//!
//! This binary fixes that. After `trunk build` produces the final `index.html`
//! (with hashed JS/CSS/wasm asset tags injected), the Trunk `post_build` hook
//! runs this tool. For each post and project it writes a static HTML file at the
//! route's path (e.g. `posts/<slug>/index.html`) that is identical to
//! `index.html` except the `<!-- META:START -->` .. `<!-- META:END -->` block is
//! rebuilt with that page's own title, description, canonical URL and Open
//! Graph tags. The file still boots the same wasm bundle, so a real visitor landing
//! there gets the full interactive SPA.
//!
//! Cloudflare Pages serves an existing static asset before consulting
//! `_redirects`, so a crawler requesting `/posts/<slug>` receives the
//! prerendered file, while unknown routes fall back to the SPA shell.

use serde::Deserialize;
use std::fs;
use std::path::Path;

const BASE_URL: &str = "https://www.skharchikov.com";
const IMAGE_URL: &str = "https://www.skharchikov.com/computer_7268855.png";
const SITE_NAME: &str = "Sergei Kharchikov";
const START: &str = "<!-- META:START -->";
const END: &str = "<!-- META:END -->";

#[derive(Deserialize)]
struct PostFrontMatter {
    title: String,
    slug: String,
    excerpt: String,
}

#[derive(Deserialize)]
struct ProjectFrontMatter {
    name: String,
    slug: String,
    description: String,
}

/// One page to prerender.
struct Page {
    /// Route path relative to site root, e.g. `posts/my-slug`.
    route: String,
    title: String,
    description: String,
    /// Open Graph type: `article` for posts, `website` for projects.
    og_type: &'static str,
}

fn main() {
    // Trunk runs hooks against the staging dir and exposes it here; fall back to
    // `dist` for manual runs.
    let dist = std::env::var("TRUNK_STAGING_DIR").unwrap_or_else(|_| "dist".to_string());
    let dist = Path::new(&dist);

    let index_path = dist.join("index.html");
    let template = fs::read_to_string(&index_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", index_path.display()));

    let start = template
        .find(START)
        .unwrap_or_else(|| panic!("`{START}` marker not found in index.html"));
    let end = template
        .find(END)
        .unwrap_or_else(|| panic!("`{END}` marker not found in index.html"));
    assert!(start < end, "META markers out of order in index.html");

    let prefix = &template[..start];
    let suffix = &template[end + END.len()..];

    let mut pages: Vec<Page> = Vec::new();

    for fm in read_frontmatter::<PostFrontMatter>("posts") {
        pages.push(Page {
            route: format!("posts/{}", fm.slug),
            title: fm.title,
            description: fm.excerpt,
            og_type: "article",
        });
    }
    for fm in read_frontmatter::<ProjectFrontMatter>("projects") {
        pages.push(Page {
            route: format!("projects/{}", fm.slug),
            title: fm.name,
            description: fm.description,
            og_type: "website",
        });
    }

    for page in &pages {
        let html = format!("{prefix}{}{suffix}", meta_block(page));
        let out_dir = dist.join(&page.route);
        fs::create_dir_all(&out_dir)
            .unwrap_or_else(|e| panic!("failed to create {}: {e}", out_dir.display()));
        let out_file = out_dir.join("index.html");
        fs::write(&out_file, html)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", out_file.display()));
    }

    println!("prerender: wrote {} static page(s)", pages.len());
}

/// Read every `*.md` in `dir`, parse its YAML frontmatter into `T`.
fn read_frontmatter<T: for<'de> Deserialize<'de>>(dir: &str) -> Vec<T> {
    let path = Path::new(dir);
    if !path.exists() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(path).unwrap_or_else(|e| panic!("failed to read {dir}: {e}")) {
        let entry = entry.expect("failed to read dir entry");
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let content =
            fs::read_to_string(&p).unwrap_or_else(|e| panic!("failed to read {}: {e}", p.display()));
        // Frontmatter is delimited by `---` markers, same as build.rs.
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            panic!("{}: missing YAML frontmatter", p.display());
        }
        let fm: T = serde_yaml::from_str(parts[1].trim())
            .unwrap_or_else(|e| panic!("{}: bad frontmatter: {e}", p.display()));
        out.push(fm);
    }
    out
}

/// Build the meta block (the content between the markers, markers included) for
/// a single page.
fn meta_block(page: &Page) -> String {
    let title = esc(&page.title);
    let desc = esc(&page.description);
    let url = format!("{BASE_URL}/{}", page.route);
    format!(
        "{START}\n    \
         <title>{title}</title>\n    \
         <meta name=\"title\" content=\"{title}\">\n    \
         <meta name=\"description\" content=\"{desc}\">\n    \
         <link rel=\"canonical\" href=\"{url}\">\n    \
         <meta property=\"og:type\" content=\"{ogtype}\">\n    \
         <meta property=\"og:url\" content=\"{url}\">\n    \
         <meta property=\"og:title\" content=\"{title}\">\n    \
         <meta property=\"og:description\" content=\"{desc}\">\n    \
         <meta property=\"og:site_name\" content=\"{site}\">\n    \
         <meta property=\"og:image\" content=\"{image}\">\n    \
         {END}",
        ogtype = page.og_type,
        site = SITE_NAME,
        image = IMAGE_URL,
    )
}

/// Escape a string for use inside a double-quoted HTML attribute.
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
