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

/// Scroll `el` just enough to be visible (no jump when it already is).
pub fn reveal(el: &web_sys::Element) {
    let opts = web_sys::ScrollIntoViewOptions::new();
    opts.set_block(web_sys::ScrollLogicalPosition::Nearest);
    el.scroll_into_view_with_scroll_into_view_options(&opts);
}

/// After a key moved the selection: if a link or button in the view has
/// focus (a row, a cover, a tag chip), move focus to the selected item among
/// `items` (a selector inside `main`). A focused link keeps Enter, so without
/// this Enter would open the focused item, not the highlighted one.
pub fn focus_follows(items: &str, i: usize) {
    let Some(active) = document().active_element() else { return };
    let in_view = active.closest("main").ok().flatten().is_some();
    if !in_view || super::current_mode() != tui_core::keys::Mode::OnControl {
        return;
    }
    let item = document().query_selector_all(&format!("main {items}")).ok().and_then(|l| l.item(i as u32));
    if let Some(el) = item.and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok()) {
        let _ = el.focus();
    }
}

/// Keyboard/mouse selectable list. A click opens the row (plain anchors: the
/// router intercepts same-origin clicks; modifier clicks open new tabs as
/// usual). Hovering or focusing a row selects it, so the preview follows the
/// pointer and Tab focus; j/k move the selection; Enter is handled by the
/// shell through the view's open target.
#[component]
pub fn SelectList(#[prop(into)] items: Signal<Vec<ListRow>>, selected: RwSignal<usize>) -> impl IntoView {
    let list = create_node_ref::<html::Ul>();

    // Keep the selected row visible when it moves (not on first render).
    create_effect(move |first: Option<()>| {
        let i = selected.get();
        if first.is_some() {
            if let Some(row) = list.get_untracked().and_then(|ul| ul.children().item(i as u32)) {
                reveal(&row);
            }
        }
    });

    view! {
        <ul class="list" node_ref=list>
            {move || {
                items
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i, row)| {
                        view! {
                            <li class:sel=move || selected.get() == i>
                                <a
                                    href=row.href.clone()
                                    class="row"
                                    aria-current=move || (selected.get() == i).then_some("true")
                                    on:mouseenter=move |_| selected.set(i)
                                    on:focus=move |_| selected.set(i)
                                >
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
