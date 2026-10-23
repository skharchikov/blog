use leptos::*;
use wasm_bindgen::JsCast;

/// One row: a left column (date / name), the title, a right column (tags),
/// and where the row leads.
#[derive(Clone, PartialEq)]
pub struct ListRow {
    pub left: String,
    pub title: String,
    pub right: String,
    pub href: String,
}

/// Keyboard/mouse selectable list. With a mouse, the first click selects and a
/// click on the selected row opens it; a tap on a touch screen opens directly.
/// Plain anchors: the router intercepts same-origin clicks (and honours
/// `preventDefault`). Keyboard "open" is handled by the view via `KeyBus`.
#[component]
pub fn SelectList(#[prop(into)] items: Signal<Vec<ListRow>>, selected: RwSignal<usize>) -> impl IntoView {
    let list = create_node_ref::<html::Ul>();

    // Keep the selected row in view when it moves (not on first render, so
    // the page doesn't jump on load).
    create_effect(move |first: Option<()>| {
        let i = selected.get();
        if first.is_some() {
            if let Some(row) = list.get_untracked().and_then(|ul| ul.children().item(i as u32)) {
                if let Ok(row) = row.dyn_into::<web_sys::HtmlElement>() {
                    row.scroll_into_view_with_bool(false);
                }
            }
        }
    });

    let coarse = window().match_media("(pointer: coarse)").ok().flatten().map(|m| m.matches()).unwrap_or(false);

    view! {
        <ul class="list" node_ref=list>
            {move || {
                items
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i, row)| {
                        let on_click = move |e: ev::MouseEvent| {
                            if !coarse && selected.get_untracked() != i {
                                e.prevent_default();
                                selected.set(i);
                            }
                        };
                        view! {
                            <li class:sel=move || selected.get() == i>
                                <a href=row.href.clone() class="row" on:click=on_click>
                                    <span class="date">{row.left.clone()}</span>
                                    <span class="t">{row.title.clone()}</span>
                                    <span class="tg">{row.right.clone()}</span>
                                </a>
                            </li>
                        }
                    })
                    .collect_view()
            }}
        </ul>
    }
}
