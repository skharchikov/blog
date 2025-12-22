use leptos::*;

#[component]
pub fn Programmer(
    #[prop(into)] visible: Signal<bool>
) -> impl IntoView {
    let img_ref = create_node_ref::<html::Img>();

    // Force GIF to loop by reloading it at random intervals
    create_effect(move |_| {
        if visible.get() {
            if let Some(img) = img_ref.get() {
                let img_clone = img.clone();

                // Recursive function to reload with random delay
                fn schedule_reload(img: leptos::HtmlElement<html::Img>) {
                    // Random delay between 5-10 seconds (5000-10000ms)
                    let random_delay = 5000 + (js_sys::Math::random() * 5000.0) as u32;

                    let img_clone = img.clone();
                    set_timeout(
                        move || {
                            let current_src = img_clone.get_attribute("src").unwrap_or_default();
                            // Toggle between v=2 and v=3 to force reload
                            let new_src = if current_src.contains("v=2") {
                                "/programmer.gif?v=3"
                            } else {
                                "/programmer.gif?v=2"
                            };
                            let _ = img_clone.set_attribute("src", new_src);

                            // Schedule next reload
                            schedule_reload(img_clone);
                        },
                        std::time::Duration::from_millis(random_delay as u64)
                    );
                }

                schedule_reload(img_clone);
            }
        }
    });

    view! {
        <div class="programmer-animation" class:visible={move || visible.get()}>
            <img
                _ref=img_ref
                src="/programmer.gif?v=2"
                alt="Programmer animation"
                class="programmer-gif"
            />
        </div>
    }
}
