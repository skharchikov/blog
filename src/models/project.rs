use std::sync::OnceLock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub github_url: String,
    pub content: String,
    pub tags: Vec<String>,
}

static PROJECTS: OnceLock<Vec<Project>> = OnceLock::new();

impl Project {
    pub fn all_projects() -> &'static Vec<Self> {
        PROJECTS.get_or_init(|| include!(concat!(env!("OUT_DIR"), "/generated_projects.rs")))
    }

    pub fn find_by_slug(slug: &str) -> Option<&'static Project> {
        Self::all_projects()
            .iter()
            .find(|project| project.slug == slug)
    }
}
