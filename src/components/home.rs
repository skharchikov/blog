use crate::components::Programmer;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let full_text = "skharchikov";
    let (typed_text, set_typed_text) = create_signal(String::new());
    let (cursor_state, set_cursor_state) = create_signal("blinking"); // "blinking", "slow-blink", "stopped"
    let (show_github, set_show_github) = create_signal(false);

    // Typing animation with realistic variable speed
    create_effect(move |_| {
        let chars: Vec<char> = full_text.chars().collect();

        spawn_local(async move {
            for (i, ch) in chars.iter().enumerate() {
                // Variable delay to simulate realistic typing
                let delay = if i == 0 {
                    400 // Longer pause before starting
                } else {
                    // Random delay between 100-250ms, with occasional longer pauses
                    let base_delay = 100 + (js_sys::Math::random() * 150.0) as i32;
                    if js_sys::Math::random() > 0.8 {
                        base_delay + 200 // Occasional longer pause
                    } else {
                        base_delay
                    }
                };

                gloo_timers::future::TimeoutFuture::new(delay as u32).await;

                let mut current = typed_text.get();
                current.push(*ch);
                set_typed_text.set(current);

                // When done: slow blink, show GitHub, then stop cursor
                if i == chars.len() - 1 {
                    gloo_timers::future::TimeoutFuture::new(500).await;
                    set_cursor_state.set("slow-blink");
                    gloo_timers::future::TimeoutFuture::new(200).await;
                    set_show_github.set(true);
                    // Let it blink slowly for ~4 seconds (2 blinks at 2s each)
                    gloo_timers::future::TimeoutFuture::new(4000).await;
                    set_cursor_state.set("stopped");
                }
            }
        });
    });

    view! {
        <div class="home-container">
            <div class="home-content">
                <div class="home-prompt" class:visible={move || !typed_text.get().is_empty()}>
                    "$ whoami"
                </div>
                <div class="home-logo-container">
                    <Programmer visible={show_github} />
                    <h1 class="home-logo">
                        {move || typed_text.get()}
                        <span
                            class="cursor gradient-caret"
                            class:slow-blink={move || cursor_state.get() == "slow-blink"}
                            class:stopped={move || cursor_state.get() == "stopped"}
                        ></span>
                    </h1>
                </div>
                <div class="home-tagline" class:visible={move || show_github.get()}>
                    <b>"Rust"</b>" · "<b>"Scala"</b>" · backend & distributed systems"
                </div>
                <div class="home-chips" class:visible={move || show_github.get()}>
                    <span class="chip hot">"Rust"</span>
                    <span class="chip">"Scala"</span>
                    <span class="chip">"async"</span>
                    <span class="chip">"low-latency"</span>
                    <span class="chip">"distributed"</span>
                </div>
            </div>
        </div>
    }
}
