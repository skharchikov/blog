use crate::tui::{Pane, ViewKind};
use leptos::*;
use leptos_router::*;
use tui_core::keys::View;
use tui_core::tabs::Tab;

/// Any path outside the site's routes (and unknown post/project slugs).
#[component]
pub fn NotFoundView() -> impl IntoView {
    expect_context::<ViewKind>().0.set(View::Plain);
    let path = use_location().pathname;
    view! {
        <Pane title="shell" focused=true class="shell">
            <div class="ln">
                <span class="ps">"me@skh"</span>" "<span class="pth">"~"</span>" $ cd "{move || path.get()}
            </div>
            <div class="out">
                <div class="ln">{move || format!("cd: {}: no such file or directory", path.get())}</div>
            </div>
            <div class="ln"><span class="ps">"me@skh"</span>" "<span class="pth">"~"</span>" $ ls"</div>
            <div class="out ls-tabs">
                {Tab::ALL.into_iter().map(|t| view! { <A href=t.path()>{format!("{}/", t.label())}</A> }).collect_view()}
            </div>
        </Pane>
    }
}
