use leptos::*;

#[derive(Clone, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub github_url: String,
}

impl Project {
    fn all_projects() -> Vec<Self> {
        vec![
            Project {
                name: "Rust Blog".to_string(),
                description: "Personal blog built with Leptos and Rust".to_string(),
                github_url: "https://github.com/skharchikov/blog".to_string(),
            },
            // Add more projects here
        ]
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = Project::all_projects();

    view! {
        <div class="projects-container">
            <h1 class="page-title">"Projects"</h1>
            <div class="projects-list">
                {projects.into_iter().map(|project| {
                    view! {
                        <div class="project-card">
                            <div class="project-header">
                                <h2 class="project-name">{&project.name}</h2>
                                <a href={&project.github_url} target="_blank" rel="noopener noreferrer" class="project-github-link">
                                    <svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" height="20" width="20" xmlns="http://www.w3.org/2000/svg">
                                        <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                                    </svg>
                                </a>
                            </div>
                            <p class="project-description">{&project.description}</p>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
