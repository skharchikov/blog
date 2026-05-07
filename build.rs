use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use syntect::html::{css_for_theme_with_class_style, ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use two_face::theme::{extra, EmbeddedThemeName};

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
    order: u32,
    name: String,
    slug: String,
    description: String,
    github_url: String,
    tags: Vec<String>,
}

fn main() {
    println!("cargo:rerun-if-changed=posts");
    println!("cargo:rerun-if-changed=projects");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let syntax_set = SyntaxSet::load_defaults_newlines();

    generate_posts(&out_dir, &syntax_set);
    generate_projects(&out_dir, &syntax_set);
    generate_highlight_css(&out_dir);
}

fn generate_posts(out_dir: &str, syntax_set: &SyntaxSet) {
    let posts_dir = Path::new("posts");
    let dest_path = Path::new(out_dir).join("generated_posts.rs");

    let mut posts_code = String::from("vec![\n");
    let mut id = 1u32;

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

    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        let content =
            fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {:?}", path));

        let (frontmatter, markdown) = parse_post_content(&content);
        let html = markdown_to_html(&markdown, syntax_set);

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
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() < 3 {
        panic!("Invalid markdown file format. Expected YAML frontmatter delimited by ---");
    }

    let frontmatter: PostFrontMatter =
        serde_yaml::from_str(parts[1].trim()).expect("Failed to parse YAML frontmatter");

    let markdown = parts[2].trim().to_string();

    (frontmatter, markdown)
}

fn parse_project_content(content: &str) -> (ProjectFrontMatter, String) {
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() < 3 {
        panic!("Invalid markdown file format. Expected YAML frontmatter delimited by ---");
    }

    let frontmatter: ProjectFrontMatter =
        serde_yaml::from_str(parts[1].trim()).expect("Failed to parse YAML frontmatter");

    let markdown = parts[2].trim().to_string();

    (frontmatter, markdown)
}

fn generate_projects(out_dir: &str, syntax_set: &SyntaxSet) {
    let projects_dir = Path::new("projects");

    if !projects_dir.exists() {
        fs::create_dir(projects_dir).expect("Failed to create projects directory");
    }

    let dest_path = Path::new(out_dir).join("generated_projects.rs");

    let mut projects_code = String::from("vec![\n");
    let mut id = 1u32;

    let entries: Vec<_> = fs::read_dir(projects_dir)
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

    let mut projects_with_order: Vec<_> = entries
        .iter()
        .map(|entry| {
            let path = entry.path();
            let content =
                fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {:?}", path));
            let (frontmatter, markdown) = parse_project_content(&content);
            (entry, frontmatter, markdown)
        })
        .collect();

    projects_with_order.sort_by_key(|(_, frontmatter, _)| frontmatter.order);

    for (_entry, frontmatter, markdown) in projects_with_order {
        let html = markdown_to_html(&markdown, syntax_set);

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

fn markdown_to_html(markdown: &str, syntax_set: &SyntaxSet) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);

    let mut events: Vec<Event> = Vec::new();
    let mut in_code = false;
    let mut code_lang = String::new();
    let mut code_buf = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code = true;
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                code_buf.clear();
            }
            Event::End(TagEnd::CodeBlock) if in_code => {
                in_code = false;
                let html = highlight_code(&code_buf, &code_lang, syntax_set);
                events.push(Event::Html(CowStr::Boxed(html.into_boxed_str())));
            }
            Event::Text(t) if in_code => {
                code_buf.push_str(&t);
            }
            e => events.push(e),
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    html_output
}

fn highlight_code(code: &str, lang: &str, syntax_set: &SyntaxSet) -> String {
    let syntax = syntax_set
        .find_syntax_by_token(lang)
        .or_else(|| syntax_set.find_syntax_by_extension(lang))
        .or_else(|| syntax_set.find_syntax_by_name(lang))
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    let mut generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, syntax_set, ClassStyle::Spaced);

    for line in LinesWithEndings::from(code) {
        let _ = generator.parse_html_for_line_which_includes_newline(line);
    }

    let highlighted = generator.finalize();
    format!("<pre class=\"code\"><code>{}</code></pre>", highlighted)
}

fn generate_highlight_css(out_dir: &str) {
    let theme_set = extra();
    let light = theme_set.get(EmbeddedThemeName::Github);
    let dark = theme_set.get(EmbeddedThemeName::DarkNeon);

    let light_css = css_for_theme_with_class_style(light, ClassStyle::Spaced)
        .expect("failed to generate light theme css");
    let dark_css = css_for_theme_with_class_style(dark, ClassStyle::Spaced)
        .expect("failed to generate dark theme css");

    let combined = format!(
        "{}\n{}",
        scope_css(&light_css, "body:not(.dark-mode)"),
        scope_css(&dark_css, "body.dark-mode"),
    );

    fs::write(Path::new(out_dir).join("highlight.css"), combined)
        .expect("failed to write highlight.css");
}

fn scope_css(css: &str, theme_scope: &str) -> String {
    const CONTENT_SCOPES: &[&str] = &[".post-content", ".project-content"];
    let mut out = String::new();
    let mut i = 0;
    while i < css.len() {
        let Some(brace_rel) = css[i..].find('{') else {
            break;
        };
        let brace = i + brace_rel;
        let sel_part = &css[i..brace];
        let Some(close_rel) = css[brace..].find('}') else {
            break;
        };
        let close = brace + close_rel;
        let body_part = &css[brace..=close];

        let selectors: Vec<String> = sel_part
            .split(',')
            .flat_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Vec::new();
                }
                CONTENT_SCOPES
                    .iter()
                    .map(|cs| format!("{} {} {}", theme_scope, cs, trimmed))
                    .collect()
            })
            .collect();

        if !selectors.is_empty() {
            out.push_str(&selectors.join(",\n"));
            out.push(' ');
            out.push_str(body_part);
            out.push('\n');
        }

        i = close + 1;
    }
    out
}

fn escape_quotes(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn escape_for_rust_string(s: &str) -> String {
    s.to_string()
}
