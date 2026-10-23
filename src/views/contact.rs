use super::EMAILS;
use crate::tui::{Pane, Scene, ViewKind};
use leptos::*;
use std::time::Duration;
use tui_core::keys::View;
use tui_core::readout;
use wasm_bindgen::{JsCast, JsValue};

/// `navigator.clipboard.writeText(text)`, via reflection so no extra web-sys
/// features are needed. Returns false where the Clipboard API is missing.
fn copy_to_clipboard(text: &str) -> bool {
    let get = |target: &JsValue, key: &str| js_sys::Reflect::get(target, &JsValue::from_str(key)).ok();
    let clipboard = get(&window().into(), "navigator").and_then(|n| get(&n, "clipboard"));
    let write = clipboard.as_ref().and_then(|c| get(c, "writeText")).and_then(|f| f.dyn_into::<js_sys::Function>().ok());
    match (clipboard, write) {
        (Some(c), Some(f)) => f.call1(&c, &JsValue::from_str(text)).is_ok(),
        _ => false,
    }
}

#[component]
fn EmailRow(address: &'static str) -> impl IntoView {
    let copied = create_rw_signal(false);
    let copy = move |_| {
        if copy_to_clipboard(address) {
            copied.set(true);
            set_timeout(move || copied.set(false), Duration::from_secs(2));
        }
    };
    view! {
        <li class="email">
            <a href=format!("mailto:{address}")>{address}</a>
            <button class="copy" on:click=copy aria-label=format!("Copy {address}")>
                {move || if copied.get() { "copied" } else { "copy" }}
            </button>
        </li>
    }
}

#[component]
pub fn ContactView() -> impl IntoView {
    expect_context::<ViewKind>().0.set(View::Plain);
    view! {
        <div class="body-grid cols contact">
            <Scene name="contact" readout=readout::CONTACT.map(String::from) />
            <Pane title="contact" focused=true>
                <p class="mut">"Email is the only way in."</p>
                <ul class="emails">{EMAILS.iter().map(|a| view! { <EmailRow address=a /> }).collect_view()}</ul>
                <p class="mut">
                    "Code lives on "
                    <a href="https://github.com/skharchikov" target="_blank" rel="noopener noreferrer">"github.com/skharchikov"</a>
                    "."
                </p>
            </Pane>
        </div>
    }
}
