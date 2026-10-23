use leptos::*;

/// A bordered TUI pane; its title sits on the top border. The focused pane
/// is drawn in the theme accent.
#[component]
pub fn Pane(
    #[prop(into)] title: String,
    #[prop(optional)] focused: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class=format!("pane {class}") class:focus=focused aria-label=title.clone()>
            <h2 class="ttl">{title}</h2>
            <div class="in">{children()}</div>
        </section>
    }
}
