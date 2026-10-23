//! The home-tab shell: parses one input line into actions for the UI.

use crate::tabs::Tab;
use crate::theme::Theme;

/// What the shell knows about the site. Posts are newest first; `projects`
/// holds project slugs, which double as their displayed names.
pub struct ShellData {
    pub posts: Vec<PostRef>,
    pub projects: Vec<String>,
    pub reading: Vec<String>,
    pub emails: &'static [&'static str],
}

pub struct PostRef {
    pub date: String,
    pub title: String,
    pub slug: String,
}

/// A piece of output text. `Accent` and `Date` only change the colour.
#[derive(Clone, Debug, PartialEq)]
pub enum Seg {
    Text(String),
    Accent(String),
    Date(String),
    Link { label: String, href: String },
}

impl Seg {
    pub fn text(&self) -> &str {
        match self {
            Seg::Text(t) | Seg::Accent(t) | Seg::Date(t) => t,
            Seg::Link { label, .. } => label,
        }
    }
}

pub type Line = Vec<Seg>;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Print(Vec<Line>),
    Navigate(String),
    SetTheme(Theme),
    Clear,
}

/// The `whoami` intro, also prefilled at the top of the transcript.
pub const WHOAMI: [&str; 3] = [
    "skh",
    "Software engineer working with Rust and Scala.",
    "I write about what I build.",
];

const COMMANDS: [&str; 8] = ["help", "whoami", "ls", "cd", "open", "theme", "email", "clear"];

const HELP: [&str; 8] = [
    "help            list the commands",
    "whoami          who runs this site",
    "ls [tab]        list writing, projects or books",
    "cd <tab>        go to a tab (cd ~ for home)",
    "open <n>        open post n from `ls writing`",
    "theme [name]    list themes or switch to one",
    "email           how to reach me",
    "clear           clear the screen",
];

fn text(s: impl Into<String>) -> Seg {
    Seg::Text(s.into())
}

fn say(lines: impl IntoIterator<Item = String>) -> Vec<Action> {
    vec![Action::Print(lines.into_iter().map(|l| vec![text(l)]).collect())]
}

/// A tab named by a `cd`/`ls` argument: tab labels, `posts`, `contacts`, `~`;
/// a trailing `/` is ignored.
fn tab_arg(arg: &str) -> Option<Tab> {
    match arg.trim_end_matches('/').to_lowercase().as_str() {
        "" | "~" | "home" => Some(Tab::Home),
        "writing" | "posts" => Some(Tab::Writing),
        "projects" => Some(Tab::Projects),
        "books" => Some(Tab::Books),
        "contact" | "contacts" => Some(Tab::Contact),
        _ => None,
    }
}

/// Run one input line. Empty input does nothing.
pub fn run(input: &str, data: &ShellData) -> Vec<Action> {
    let mut words = input.split_whitespace();
    let Some(cmd) = words.next() else { return Vec::new() };
    let cmd = cmd.to_lowercase();
    let arg = words.collect::<Vec<_>>().join(" ");

    match cmd.as_str() {
        "help" => say(HELP.map(String::from)),
        "whoami" => vec![Action::Print(
            WHOAMI.iter().enumerate().map(|(i, l)| vec![if i == 0 { Seg::Accent(l.to_string()) } else { text(*l) }]).collect(),
        )],
        "cd" => match tab_arg(&arg) {
            Some(tab) => vec![Action::Navigate(tab.path().to_string())],
            None => say([format!("cd: {arg}: no such tab")]),
        },
        "ls" => ls(&arg, data),
        "open" => match arg.parse::<usize>().ok().and_then(|n| n.checked_sub(1)).and_then(|i| data.posts.get(i)) {
            Some(p) => vec![Action::Navigate(format!("/posts/{}", p.slug))],
            None => say(["open: try `ls writing`, then `open 1`".to_string()]),
        },
        "theme" => {
            if arg.is_empty() {
                let names: Vec<_> = Theme::ALL.iter().map(|t| t.name()).collect();
                say([format!("themes: {}", names.join(", "))])
            } else {
                match Theme::from_name(&arg) {
                    Some(t) => vec![Action::SetTheme(t)],
                    None => say([format!("theme: unknown \"{arg}\"")]),
                }
            }
        }
        "email" => vec![Action::Print(
            data.emails.iter().map(|e| vec![Seg::Link { label: e.to_string(), href: format!("mailto:{e}") }]).collect(),
        )],
        "clear" => vec![Action::Clear],
        _ => say([format!("{cmd}: command not found. try help")]),
    }
}

fn ls(arg: &str, data: &ShellData) -> Vec<Action> {
    let lines: Vec<Line> = match (arg.is_empty(), tab_arg(arg)) {
        (true, _) => vec![Tab::ALL[1..]
            .iter()
            .flat_map(|t| [Seg::Link { label: format!("{}/", t.label()), href: t.path().to_string() }, text("  ")])
            .collect()],
        (false, Some(Tab::Writing)) => data
            .posts
            .iter()
            .enumerate()
            .map(|(i, p)| {
                vec![
                    text(format!("{}  ", i + 1)),
                    Seg::Date(p.date.clone()),
                    text("  "),
                    Seg::Link { label: p.title.clone(), href: format!("/posts/{}", p.slug) },
                ]
            })
            .collect(),
        (false, Some(Tab::Projects)) => data
            .projects
            .iter()
            .map(|slug| vec![Seg::Link { label: format!("{slug}/"), href: format!("/projects/{slug}") }])
            .collect(),
        (false, Some(Tab::Books)) => data.reading.iter().map(|t| vec![text(format!("reading: {t}"))]).collect(),
        _ => vec![vec![text(format!("ls: {arg}: no such directory"))]],
    };
    vec![Action::Print(lines)]
}

/// Complete a command, or the tab/theme argument of `cd`, `ls` and `theme`,
/// when exactly one candidate matches.
pub fn complete(prefix: &str) -> Option<String> {
    let unique = |candidates: &[&str], p: &str| {
        let hits: Vec<_> = candidates.iter().filter(|c| c.starts_with(p)).collect();
        (hits.len() == 1).then(|| hits[0].to_string())
    };
    match prefix.split_once(' ') {
        None => unique(&COMMANDS, &prefix.to_lowercase()),
        Some((cmd, arg)) => {
            let tabs: Vec<&str> = Tab::ALL.iter().map(|t| t.label()).collect();
            let themes: Vec<&str> = Theme::ALL.iter().map(|t| t.name()).collect();
            let pool = match cmd { "cd" | "ls" => tabs, "theme" => themes, _ => return None };
            unique(&pool, arg.trim()).map(|a| format!("{cmd} {a}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    fn fixture() -> ShellData {
        let post = |date: &str, title: &str, slug: &str| PostRef { date: date.into(), title: title.into(), slug: slug.into() };
        ShellData {
            posts: vec![
                post("2026-07-02", "Blind 75 in Rust: Two Sum", "blind75-two-sum"),
                post("2026-06-24", "Blind 75 in Rust: Valid Anagram", "blind75-valid-anagram"),
                post("2026-06-23", "Blind 75 in Rust: Contains Duplicate", "blind75-contains-duplicate"),
                post("2026-05-05", "A Polymarket Copy Trader in Rust", "copy-trading-bot-rust"),
                post("2025-12-22", "Building a Tiny Retro PC with Raspberry Pi 4", "building-tiny-retro-pc"),
            ],
            projects: ["rig-openapi-tools", "rust-blog", "polymarket-bot", "github-stats-rs", "blackjack"].map(String::from).to_vec(),
            reading: ["Extreme Privacy", "The Stand", "Rust for Rustaceans"].map(String::from).to_vec(),
            emails: &["me@skh.rs"],
        }
    }

    fn printed(actions: &[Action]) -> String {
        let mut lines = Vec::new();
        for a in actions {
            if let Action::Print(ls) = a {
                for l in ls {
                    lines.push(l.iter().map(Seg::text).collect::<String>());
                }
            }
        }
        lines.join("\n")
    }

    #[test] fn empty_and_spaces_do_nothing() { assert!(run("", &fixture()).is_empty()); assert!(run("   ", &fixture()).is_empty()); }
    #[test] fn unknown_command() { assert_eq!(printed(&run("rm -rf /", &fixture())), "rm: command not found. try help"); }
    #[test] fn case_and_spacing_tolerated() { assert_eq!(run("  CD   books ", &fixture()), vec![Action::Navigate("/books".into())]); }
    #[test] fn cd_targets() {
        assert_eq!(run("cd ~", &fixture()), vec![Action::Navigate("/".into())]);
        assert_eq!(run("cd", &fixture()), vec![Action::Navigate("/".into())]);
        assert_eq!(run("cd writing/", &fixture()), vec![Action::Navigate("/posts".into())]);
        assert_eq!(printed(&run("cd foo", &fixture())), "cd: foo: no such tab");
    }
    #[test] fn ls_writing_numbered_links() {
        let p = printed(&run("ls writing/", &fixture()));
        assert!(p.starts_with("1  2026-07-02  Blind 75 in Rust: Two Sum"), "{p}");
        assert_eq!(p.lines().count(), 5);
    }
    #[test] fn open_bounds() {
        assert_eq!(run("open 1", &fixture()), vec![Action::Navigate("/posts/blind75-two-sum".into())]);
        for bad in ["open 0", "open 99", "open x", "open"] {
            assert_eq!(printed(&run(bad, &fixture())), "open: try `ls writing`, then `open 1`");
        }
    }
    #[test] fn theme_cmd() {
        assert_eq!(run("theme gruvbox", &fixture()), vec![Action::SetTheme(Theme::Gruvbox)]);
        assert_eq!(printed(&run("theme", &fixture())), "themes: skh, tokyo-night, gruvbox, paper");
        assert_eq!(printed(&run("theme neon", &fixture())), "theme: unknown \"neon\"");
    }
    #[test] fn email_lists_the_one_address() { assert_eq!(printed(&run("email", &fixture())), "me@skh.rs"); }
    #[test] fn whoami_is_the_short_intro() {
        assert_eq!(printed(&run("whoami", &fixture())), "skh\nSoftware engineer working with Rust and Scala.\nI write about what I build.");
    }
    #[test] fn clear_and_help() { assert_eq!(run("clear", &fixture()), vec![Action::Clear]); assert!(printed(&run("help", &fixture())).contains("open <n>")); }
    #[test] fn completion() { assert_eq!(complete("th"), Some("theme".into())); assert_eq!(complete("cd bo"), Some("cd books".into())); assert_eq!(complete("zz"), None); }
}
