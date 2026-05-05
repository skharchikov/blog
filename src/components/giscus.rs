use leptos::*;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

const REPO: &str = "skharchikov/blog";
const REPO_ID: &str = "R_kgDOQs7QAQ";
const CATEGORY: &str = "General";
const CATEGORY_ID: &str = "DIC_kwDOQs7QAc4C8ZMb";

fn theme_name(is_dark: bool) -> &'static str {
    if is_dark { "dark" } else { "light" }
}

fn current_dark_mode() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
        .map(|b| b.class_list().contains("dark-mode"))
        .unwrap_or_else(|| {
            web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
                .and_then(|s| s.get_item("darkMode").ok().flatten())
                .map(|v| v == "true")
                .unwrap_or(false)
        })
}

fn post_theme_to_giscus(theme: &str) {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let Ok(Some(iframe)) = document.query_selector("iframe.giscus-frame") else {
        return;
    };
    let Ok(iframe) = iframe.dyn_into::<web_sys::HtmlIFrameElement>() else {
        return;
    };
    let Some(content_window) = iframe.content_window() else {
        return;
    };

    let set_config = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&set_config, &JsValue::from_str("theme"), &JsValue::from_str(theme));
    let inner = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&inner, &JsValue::from_str("setConfig"), &set_config);
    let outer = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&outer, &JsValue::from_str("giscus"), &inner);

    let _ = content_window.post_message(&outer, "https://giscus.app");
}

#[component]
pub fn Giscus() -> impl IntoView {
    let container_ref = create_node_ref::<html::Div>();
    let (dark_mode, set_dark_mode) = create_signal(current_dark_mode());

    // Poll body.class_list for "dark-mode" toggles, mirroring the existing
    // pattern used by the Christmas tree component.
    create_effect(move |_| {
        spawn_local(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(500).await;
                let has_dark = current_dark_mode();
                if has_dark != dark_mode.get_untracked() {
                    set_dark_mode.set(has_dark);
                }
            }
        });
    });

    let mounted = Rc::new(Cell::new(false));

    create_effect(move |_| {
        let is_dark = dark_mode.get();

        if mounted.get() {
            post_theme_to_giscus(theme_name(is_dark));
            return;
        }

        let Some(container) = container_ref.get() else {
            return;
        };

        container.set_inner_html("");

        let document = web_sys::window().and_then(|w| w.document());
        let Some(document) = document else { return };

        let Ok(script) = document.create_element("script") else {
            return;
        };
        let Ok(script) = script.dyn_into::<web_sys::HtmlScriptElement>() else {
            return;
        };

        script.set_src("https://giscus.app/client.js");
        script.set_async(true);
        let _ = script.set_attribute("crossorigin", "anonymous");
        let _ = script.set_attribute("data-repo", REPO);
        let _ = script.set_attribute("data-repo-id", REPO_ID);
        let _ = script.set_attribute("data-category", CATEGORY);
        let _ = script.set_attribute("data-category-id", CATEGORY_ID);
        let _ = script.set_attribute("data-mapping", "url");
        let _ = script.set_attribute("data-strict", "0");
        let _ = script.set_attribute("data-reactions-enabled", "1");
        let _ = script.set_attribute("data-emit-metadata", "0");
        let _ = script.set_attribute("data-input-position", "bottom");
        let _ = script.set_attribute("data-theme", theme_name(is_dark));
        let _ = script.set_attribute("data-lang", "en");

        let _ = container.append_child(&script);
        mounted.set(true);
    });

    view! {
        <section class="giscus-wrapper">
            <h2 class="giscus-heading">"Comments"</h2>
            <div class="giscus" node_ref=container_ref></div>
        </section>
    }
}
