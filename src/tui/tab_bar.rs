use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use tui_core::tabs::Tab;
use tui_core::theme::Theme;

/// zellij-style tab bar. `<A>` sets `aria-current="page"` on the active tab;
/// home is matched exactly so it isn't active on every route.
#[component]
pub fn TabBar() -> impl IntoView {
    let theme = expect_context::<RwSignal<Theme>>();
    let bar = create_node_ref::<html::Header>();
    let path = use_location().pathname;

    // On narrow screens the bar scrolls sideways; keep the active tab visible.
    create_effect(move |_| {
        path.track();
        let Some(bar) = bar.get() else { return };
        request_animation_frame(move || {
            let active = bar.query_selector(".tab[aria-current=\"page\"]").ok().flatten();
            if let Some(tab) = active.and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok()) {
                bar.set_scroll_left((tab.offset_left() - 80).max(0));
            }
        });
    });

    view! {
        <header class="tabs" node_ref=bar>
            <span class="sess">"skh"</span>
            <nav class="tab-list" aria-label="Sections">
                {Tab::ALL
                    .into_iter()
                    .map(|tab| {
                        view! {
                            <A href=tab.path() class="tab" exact=(tab == Tab::Home)>
                                <span class="n">{tab.number()}</span>
                                {tab.label()}
                            </A>
                        }
                    })
                    .collect_view()}
            </nav>
            <span class="theme-name">{move || theme.get().name()}</span>
        </header>
    }
}
