use super::Pane;
use leptos::*;

/// Native sizes of the scenes that have an animation. Mirrors
/// public/sprites/manifest.json (80 frames, 10 columns × 8 rows each).
struct Sprite {
    width: u32,
    height: u32,
    label: &'static str,
}

fn sprite(name: &str) -> Option<Sprite> {
    match name {
        "home" => Some(Sprite { width: 131, height: 119, label: "skh typing at a desk with a monitor, laptop and rubber duck" }),
        "books" => Some(Sprite { width: 149, height: 115, label: "skh reading on a green sofa with two cats" }),
        _ => None,
    }
}

/// The scene pane at the top of a tab's left column: the tab's pixel-art
/// animation, or — until it has one — a stats readout in the same space.
#[component]
pub fn Scene(name: &'static str, #[prop(optional)] readout: Option<[String; 3]>) -> impl IntoView {
    let body = match sprite(name) {
        Some(s) => {
            let style = format!(
                "--sheet:url(/sprites/{name}.png);--still:url(/sprites/{name}-still.png);--w:{}px;--h:{}px",
                s.width, s.height
            );
            view! { <div class="sprite" role="img" aria-label=s.label style=style></div> }.into_view()
        }
        None => view! {
            <ul class="readout">
                {readout.unwrap_or_default().into_iter().map(|line| view! { <li>{line}</li> }).collect_view()}
            </ul>
        }
        .into_view(),
    };
    view! { <Pane title=format!("me@skh: {name}") class="scene">{body}</Pane> }
}
