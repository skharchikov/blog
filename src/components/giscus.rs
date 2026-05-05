use leptos::*;
use wasm_bindgen::JsCast;

const REPO: &str = "skharchikov/blog";
const REPO_ID: &str = "R_kgDOQs7QAQ";
const CATEGORY: &str = "General";
const CATEGORY_ID: &str = "DIC_kwDOQs7QAc4C8ZMb";

#[component]
pub fn Giscus() -> impl IntoView {
    let container_ref = create_node_ref::<html::Div>();

    create_effect(move |_| {
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
        let _ = script.set_attribute("data-theme", "dark");
        let _ = script.set_attribute("data-lang", "en");

        let _ = container.append_child(&script);
    });

    view! {
        <section class="giscus-wrapper">
            <h2 class="giscus-heading">"Comments"</h2>
            <div class="giscus" node_ref=container_ref></div>
        </section>
    }
}
