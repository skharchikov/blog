//! The five tabs and how they map to the site's routes.

/// A top-level tab. Its route is fixed; posts and projects own their sub-paths.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Home,
    Writing,
    Projects,
    Books,
    Contact,
}

impl Tab {
    pub const ALL: [Tab; 5] = [Tab::Home, Tab::Writing, Tab::Projects, Tab::Books, Tab::Contact];

    pub fn label(self) -> &'static str {
        match self {
            Tab::Home => "home",
            Tab::Writing => "writing",
            Tab::Projects => "projects",
            Tab::Books => "books",
            Tab::Contact => "contact",
        }
    }

    pub fn path(self) -> &'static str {
        match self {
            Tab::Home => "/",
            Tab::Writing => "/posts",
            Tab::Projects => "/projects",
            Tab::Books => "/books",
            Tab::Contact => "/contacts",
        }
    }

    /// 1-based position, as shown in the tab bar and used by the number keys.
    pub fn number(self) -> u8 {
        Tab::ALL.iter().position(|t| *t == self).unwrap_or(0) as u8 + 1
    }

    pub fn from_number(n: u8) -> Option<Tab> {
        n.checked_sub(1).and_then(|i| Tab::ALL.get(i as usize).copied())
    }

    /// The tab that owns a path (query string and fragment ignored), or `None`
    /// for paths outside the site's routes.
    pub fn from_path(path: &str) -> Option<Tab> {
        let path = path.split(['?', '#']).next().unwrap_or("");
        let first = path.trim_start_matches('/').split('/').next().unwrap_or("");
        match first {
            "" => Some(Tab::Home),
            "posts" => Some(Tab::Writing),
            "projects" => Some(Tab::Projects),
            "books" => Some(Tab::Books),
            "contacts" => Some(Tab::Contact),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths() {
        assert_eq!(Tab::from_path("/"), Some(Tab::Home));
        assert_eq!(Tab::from_path("/posts/blind75-two-sum"), Some(Tab::Writing));
        assert_eq!(Tab::from_path("/posts/"), Some(Tab::Writing));
        assert_eq!(Tab::from_path("/posts?tag=rust"), Some(Tab::Writing));
        assert_eq!(Tab::from_path("/projects/blackjack"), Some(Tab::Projects));
        assert_eq!(Tab::from_path("/books/"), Some(Tab::Books));
        assert_eq!(Tab::from_path("/contacts"), Some(Tab::Contact));
        assert_eq!(Tab::from_path("/postsx"), None);
        assert_eq!(Tab::from_path("/nope"), None);
    }

    #[test]
    fn numbers() {
        for t in Tab::ALL {
            assert_eq!(Tab::from_number(t.number()), Some(t));
        }
        assert_eq!(Tab::from_number(0), None);
        assert_eq!(Tab::from_number(6), None);
    }
}
