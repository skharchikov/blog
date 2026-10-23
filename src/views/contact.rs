use super::EMAILS;
use crate::tui::{Pane, Scene, ViewKind};
use leptos::*;
use tui_core::keys::View;

#[component]
pub fn ContactView() -> impl IntoView {
    expect_context::<ViewKind>().0.set(View::Plain);
    view! {
        <div class="body-grid cols contact">
            <Scene name="contact" />
            <Pane title="contact" focused=true>
                <p class="mut">"Email is the only way in."</p>
                <ul class="emails">
                    {EMAILS
                        .iter()
                        .map(|a| view! { <li class="email"><a href=format!("mailto:{a}")>{*a}</a></li> })
                        .collect_view()}
                </ul>
                <p class="mut">
                    "Code lives on "
                    <a href="https://github.com/skharchikov" target="_blank" rel="noopener noreferrer">"github.com/skharchikov"</a>
                    "."
                </p>
            </Pane>
        </div>
    }
}
