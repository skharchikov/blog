use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct PostFrontMatter {
    title: String,
    date: String,
    slug: String,
    excerpt: String,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProjectFrontMatter {
    name: String,
    slug: String,
    description: String,
    github_url: String,
    tags: Vec<String>,
}

fn main() {
    // Tell Cargo to rerun if posts or projects directory changes
    println!("cargo:rerun-if-changed=posts");
    println!("cargo:rerun-if-changed=projects");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    // Generate posts
    generate_posts(&out_dir);

    // Generate projects
    generate_projects(&out_dir);
}

fn generate_posts(out_dir: &str) {
    let posts_dir = Path::new("posts");
    let dest_path = Path::new(out_dir).join("generated_posts.rs");

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
        let (frontmatter, markdown) = parse_post_content(&content);

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

fn parse_post_content(content: &str) -> (PostFrontMatter, String) {
    // Split frontmatter from markdown
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() < 3 {
        panic!("Invalid markdown file format. Expected YAML frontmatter delimited by ---");
    }

    let frontmatter: PostFrontMatter = serde_yaml::from_str(parts[1].trim())
        .expect("Failed to parse YAML frontmatter");

    let markdown = parts[2].trim().to_string();

    (frontmatter, markdown)
}

fn parse_project_content(content: &str) -> (ProjectFrontMatter, String) {
    // Split frontmatter from markdown
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() < 3 {
        panic!("Invalid markdown file format. Expected YAML frontmatter delimited by ---");
    }

    let frontmatter: ProjectFrontMatter = serde_yaml::from_str(parts[1].trim())
        .expect("Failed to parse YAML frontmatter");

    let markdown = parts[2].trim().to_string();

    (frontmatter, markdown)
}

fn generate_projects(out_dir: &str) {
    let projects_dir = Path::new("projects");

    // Create projects directory if it doesn't exist
    if !projects_dir.exists() {
        fs::create_dir(projects_dir).expect("Failed to create projects directory");
    }

    let dest_path = Path::new(out_dir).join("generated_projects.rs");

    let mut projects_code = String::from("vec![\n");
    let mut id = 1u32;

    // Read all .md files from projects directory
    let mut entries: Vec<_> = fs::read_dir(projects_dir)
        .expect("Failed to read projects directory")
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
        let (frontmatter, markdown) = parse_project_content(&content);

        // Convert markdown to HTML
        let html = markdown_to_html(&markdown);

        // Generate Rust code for this project
        let escaped_content = escape_for_rust_string(&html);
        let tags_code = frontmatter
            .tags
            .iter()
            .map(|t| format!("\"{}\"", escape_quotes(t)))
            .collect::<Vec<_>>()
            .join(", ");

        projects_code.push_str(&format!(
            "    Project {{\n        id: {},\n        name: \"{}\".to_string(),\n        slug: \"{}\".to_string(),\n        description: \"{}\".to_string(),\n        github_url: \"{}\".to_string(),\n        content: r###\"{}\"###.to_string(),\n        tags: vec![{}].into_iter().map(|s| s.to_string()).collect(),\n    }},\n",
            id,
            escape_quotes(&frontmatter.name),
            escape_quotes(&frontmatter.slug),
            escape_quotes(&frontmatter.description),
            escape_quotes(&frontmatter.github_url),
            escaped_content,
            tags_code
        ));

        id += 1;
    }

    projects_code.push_str("]\n");

    fs::write(&dest_path, projects_code).expect("Failed to write generated projects");
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
