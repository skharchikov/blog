use gloo_net::http::Request;
use gloo_timers::future::TimeoutFuture;
use leptos::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;

/// GoatCounter site code — the subdomain of your goatcounter.com account.
/// Must match the `data-goatcounter` host in `index.html`.
const GOATCOUNTER_CODE: &str = "skh";

fn base_url() -> String {
    format!("https://{GOATCOUNTER_CODE}.goatcounter.com")
}

/// Fire one view hit at GoatCounter for `path` by calling `window.goatcounter.count`.
/// Returns false if the count.js script hasn't loaded yet (so the caller can retry).
fn try_count(path: &str) -> bool {
    let Some(win) = web_sys::window() else {
        return false;
    };
    let Ok(gc) = js_sys::Reflect::get(&win, &JsValue::from_str("goatcounter")) else {
        return false;
    };
    if gc.is_undefined() || gc.is_null() {
        return false;
    }
    let Ok(count_fn) = js_sys::Reflect::get(&gc, &JsValue::from_str("count")) else {
        return false;
    };
    let Ok(count_fn) = count_fn.dyn_into::<js_sys::Function>() else {
        return false;
    };

    let arg = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&arg, &JsValue::from_str("path"), &JsValue::from_str(path));
    count_fn.call1(&gc, &arg).is_ok()
}

/// count.js loads async; retry for a few seconds until `window.goatcounter` exists.
fn count_view(path: String) {
    spawn_local(async move {
        for _ in 0..20 {
            if try_count(&path) {
                return;
            }
            TimeoutFuture::new(250).await;
        }
    });
}

#[derive(serde::Deserialize)]
struct CounterResponse {
    /// Unique visitors, formatted with thousands separators, e.g. "1,234".
    count_unique: String,
}

/// Read the unique-visitor count for `path` from GoatCounter's public counter endpoint.
/// Requires "Allow adding visitor counts to pages" enabled in GoatCounter site settings.
async fn fetch_unique(path: &str) -> Option<u64> {
    // The path keeps its leading slash, producing the documented `/counter//posts/foo.json`.
    let url = format!("{}/counter/{}.json", base_url(), path);
    let resp = Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let body: CounterResponse = resp.json().await.ok()?;
    body.count_unique.replace(',', "").parse::<u64>().ok()
}

/// Honest, server-side view counter for a single page.
///
/// On mount it records one hit (GoatCounter dedupes per-visitor-per-day and filters
/// bots server-side), then displays the unique-visitor total.
#[component]
pub fn ViewCounter(#[prop(into)] path: String) -> impl IntoView {
    // Record this visit (server-side dedup keeps it honest across refreshes).
    count_view(path.clone());

    let count = create_local_resource(
        || (),
        move |_| {
            let path = path.clone();
            async move { fetch_unique(&path).await }
        },
    );

    view! {
        <span class="post-views" title="Unique visitors, counted server-side via GoatCounter">
            {move || match count.get() {
                Some(Some(n)) => format!("{n} views"),
                Some(None) => "— views".to_string(),
                None => "… views".to_string(),
            }}
        </span>
    }
}
