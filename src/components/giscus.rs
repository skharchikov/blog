use leptos::*;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

const REPO: &str = "skharchikov/blog";
const REPO_ID: &str = "R_kgDOQs7QAQ";
const CATEGORY: &str = "General";
const CATEGORY_ID: &str = "DIC_kwDOQs7QAc4C8ZMb";

fn theme_name(is_dark: bool) -> &'static str {
    if is_dark {
        "dark"
    } else {
        "light"
    }
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
    let _ = js_sys::Reflect::set(
        &set_config,
        &JsValue::from_str("theme"),
        &JsValue::from_str(theme),
    );
    let inner = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&inner, &JsValue::from_str("setConfig"), &set_config);
    let outer = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&outer, &JsValue::from_str("giscus"), &inner);

    let _ = content_window.post_message(&outer, "https://giscus.app");
}

fn inject_script(container: &web_sys::HtmlElement, is_dark: bool) {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
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
    let _ = script.set_attribute("data-loading", "lazy");

    let _ = container.append_child(&script);
}

#[component]
pub fn Giscus(#[prop(into)] dark_mode: Signal<bool>) -> impl IntoView {
    let container_ref = create_node_ref::<html::Div>();
    let mounted = Rc::new(Cell::new(false));
    let initialized = Rc::new(Cell::new(false));
    let listener_attached = Rc::new(Cell::new(false));

    {
        let mounted = mounted.clone();
        create_effect(move |_| {
            let is_dark = dark_mode.get();
            if mounted.get() {
                post_theme_to_giscus(theme_name(is_dark));
            }
        });
    }

    {
        let mounted = mounted.clone();
        let listener_attached = listener_attached.clone();
        create_effect(move |_| {
            if listener_attached.get() {
                return;
            }
            let Some(window) = web_sys::window() else {
                return;
            };
            listener_attached.set(true);

            let mounted_cb = mounted.clone();
            let cb = Closure::<dyn FnMut(JsValue)>::new(move |event: JsValue| {
                let origin = js_sys::Reflect::get(&event, &JsValue::from_str("origin"))
                    .ok()
                    .and_then(|v| v.as_string())
                    .unwrap_or_default();
                if origin == "https://giscus.app" {
                    mounted_cb.set(true);
                    post_theme_to_giscus(theme_name(dark_mode.get_untracked()));
                }
            });
            let _ = window
                .add_event_listener_with_callback("message", cb.as_ref().unchecked_ref());
            cb.forget();
        });
    }

    {
        let mounted = mounted.clone();
        let initialized = initialized.clone();
        create_effect(move |_| {
            if initialized.get() {
                return;
            }
            let Some(container) = container_ref.get() else {
                return;
            };
            initialized.set(true);

            let element: web_sys::Element =
                JsValue::from(&*container).unchecked_into();
            let container_html: web_sys::HtmlElement =
                JsValue::from(&*container).unchecked_into();
            let mounted_cb = mounted.clone();

            let callback = Closure::<dyn FnMut(JsValue, JsValue)>::new(
                move |entries: JsValue, observer: JsValue| {
                    let entries: js_sys::Array = entries.unchecked_into();
                    let any_intersecting = entries.iter().any(|e| {
                        e.dyn_into::<web_sys::IntersectionObserverEntry>()
                            .map(|entry| entry.is_intersecting())
                            .unwrap_or(false)
                    });
                    if any_intersecting && !mounted_cb.get() {
                        let is_dark = dark_mode.get_untracked();
                        inject_script(&container_html, is_dark);
                        mounted_cb.set(true);
                        if let Ok(observer) =
                            observer.dyn_into::<web_sys::IntersectionObserver>()
                        {
                            observer.disconnect();
                        }
                    }
                },
            );

            let init = web_sys::IntersectionObserverInit::new();
            init.set_root_margin("400px 0px");

            if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &init,
            ) {
                observer.observe(&element);
            }

            callback.forget();
        });
    }

    view! {
        <section class="giscus-wrapper">
            <div class="giscus" node_ref=container_ref></div>
        </section>
    }
}
