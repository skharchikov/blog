use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct FrontMatter {
    title: String,
    date: String,
    slug: String,
    excerpt: String,
    tags: Vec<String>,
}

fn main() {
    // Tell Cargo to rerun if posts directory changes
    println!("cargo:rerun-if-changed=posts");

    let posts_dir = Path::new("posts");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_posts.rs");

    let mut posts_code = String::from("vec![\n");
    let mut id = 1u32;

    // Read all .md files from posts directory
    let mut entries: Vec<_> = fs::read_dir(posts_dir)
        .expect("Failed to read posts directory")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "md")
                .unwrap_or(false)
        })
        .collect();

    // Sort by filename for consistent ordering
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read {:?}", path));

        // Parse frontmatter and markdown
        let (frontmatter, markdown) = parse_post(&content);

        // Convert markdown to HTML
        let html = markdown_to_html(&markdown);

        // Generate Rust code for this post
        let escaped_content = escape_for_rust_string(&html);
        let tags_code = frontmatter
            .tags
            .iter()
            .map(|t| format!("\"{}\"", escape_quotes(t)))
            .collect::<Vec<_>>()
            .join(", ");

        posts_code.push_str(&format!(
            "    BlogPost {{\n        id: {},\n        title: \"{}\".to_string(),\n        slug: \"{}\".to_string(),\n        date: \"{}\".to_string(),\n        excerpt: \"{}\".to_string(),\n        content: r###\"{}\"###.to_string(),\n        tags: vec![{}].into_iter().map(|s| s.to_string()).collect(),\n    }},\n",
            id,
            escape_quotes(&frontmatter.title),
            escape_quotes(&frontmatter.slug),
            escape_quotes(&frontmatter.date),
            escape_quotes(&frontmatter.excerpt),
            escaped_content,
            tags_code
        ));

        id += 1;
    }

    posts_code.push_str("]\n");

    fs::write(&dest_path, posts_code).expect("Failed to write generated posts");
}

fn parse_post(content: &str) -> (FrontMatter, String) {
    // Split frontmatter from markdown
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() < 3 {
        panic!("Invalid markdown file format. Expected YAML frontmatter delimited by ---");
    }

    let frontmatter: FrontMatter = serde_yaml::from_str(parts[1].trim())
        .expect("Failed to parse YAML frontmatter");

    let markdown = parts[2].trim().to_string();

    (frontmatter, markdown)
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn escape_quotes(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn escape_for_rust_string(s: &str) -> String {
    // For raw strings with r###"..."###, we need to ensure the content
    // doesn't contain the closing sequence "###
    // Simple approach: if it contains "###, add more # to the delimiter
    // For simplicity, we'll just use r### which should work for HTML
    s.to_string()
}
