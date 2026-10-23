//! Stats readouts shown in the scene pane of tabs that have no animation yet.

/// Languages recognised in project tags, in display order.
const LANGS: [&str; 5] = ["rust", "scala", "python", "go", "typescript"];

pub const CONTACT: [&str; 3] = ["inbox: open", "usual reply: a few days", "no socials, email only"];

/// `N posts · M tags`, the newest date, and the most used tag (ties go to the
/// alphabetically first). Posts are `(date YYYY-MM-DD, tags)`.
pub fn writing(posts: &[(&str, &[&str])]) -> [String; 3] {
    let mut counts: Vec<(&str, usize)> = Vec::new();
    for (_, tags) in posts {
        for tag in *tags {
            match counts.iter_mut().find(|(t, _)| t == tag) {
                Some((_, n)) => *n += 1,
                None => counts.push((tag, 1)),
            }
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let last = posts.iter().map(|(d, _)| *d).max().unwrap_or("-");
    let top = counts.first().map_or("-".to_string(), |(t, _)| format!("#{t}"));
    [format!("{} posts · {} tags", posts.len(), counts.len()), format!("last: {last}"), format!("most used: {top}")]
}

/// `N repos`, the languages found among the tags, and the top project (first
/// by frontmatter `order`).
pub fn projects(names: &[&str], tags: &[&[&str]]) -> [String; 3] {
    let langs: Vec<&str> = LANGS.into_iter().filter(|l| tags.iter().any(|ts| ts.contains(l))).collect();
    let langs = if langs.is_empty() { "-".to_string() } else { langs.join(", ") };
    [format!("{} repos", names.len()), format!("langs: {langs}"), format!("top: {}", names.first().unwrap_or(&"-"))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writing_readout() {
        let r = writing(&[("2026-07-02", &["rust", "leetcode"]), ("2026-06-24", &["rust"]), ("2025-12-22", &["diy"])]);
        assert_eq!(r, ["3 posts · 3 tags".to_string(), "last: 2026-07-02".into(), "most used: #rust".into()]);
    }

    #[test]
    fn projects_readout() {
        let r = projects(&["rig-openapi-tools", "blackjack"], &[&["rust", "llm"], &["rust", "axum"]]);
        assert_eq!(r, ["2 repos".to_string(), "langs: rust".into(), "top: rig-openapi-tools".into()]);
    }

    #[test]
    fn contact_readout() {
        assert_eq!(CONTACT, ["inbox: open", "usual reply: a few days", "no socials, email only"]);
    }
}
