use leptos::*;
use wasm_bindgen::JsCast;

// TODO: replace with values from https://giscus.app after enabling Discussions
// on github.com/skharchikov/blog and installing the giscus GitHub app.
const REPO: &str = "skharchikov/blog";
const REPO_ID: &str = "REPLACE_WITH_REPO_ID";
const CATEGORY: &str = "General";
const CATEGORY_ID: &str = "REPLACE_WITH_CATEGORY_ID";

#[component]
pub fn Giscus(#[prop(into)] term: String) -> impl IntoView {
    let container_ref = create_node_ref::<html::Div>();

    create_effect(move |_| {
        let Some(container) = container_ref.get() else {
            return;
        };

        // Clear any previous giscus iframe (e.g. when navigating between posts).
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
        let _ = script.set_attribute("data-mapping", "specific");
        let _ = script.set_attribute("data-term", &term);
        let _ = script.set_attribute("data-strict", "1");
        let _ = script.set_attribute("data-reactions-enabled", "1");
        let _ = script.set_attribute("data-emit-metadata", "0");
        let _ = script.set_attribute("data-input-position", "bottom");
        let _ = script.set_attribute("data-theme", "preferred_color_scheme");
        let _ = script.set_attribute("data-lang", "en");
        let _ = script.set_attribute("data-loading", "lazy");

        let _ = container.append_child(&script);
    });

    view! {
        <section class="giscus-wrapper">
            <h2 class="giscus-heading">"Comments"</h2>
            <div class="giscus" node_ref=container_ref></div>
        </section>
    }
}
