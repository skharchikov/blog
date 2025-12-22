use std::sync::OnceLock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlogPost {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub date: String,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
}

static POSTS: OnceLock<Vec<BlogPost>> = OnceLock::new();

impl BlogPost {
    pub fn all_posts() -> &'static Vec<Self> {
        POSTS.get_or_init(|| include!(concat!(env!("OUT_DIR"), "/generated_posts.rs")))
    }

    pub fn find_by_slug(slug: &str) -> Option<&'static BlogPost> {
        Self::all_posts().iter().find(|post| post.slug == slug)
    }
}
