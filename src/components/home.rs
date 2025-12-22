use leptos::*;
use crate::components::Programmer;

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
                <Programmer visible={show_github} />
                <h1 class="home-logo">
                    {move || typed_text.get()}
                    <span
                        class="cursor"
                        class:slow-blink={move || cursor_state.get() == "slow-blink"}
                        class:stopped={move || cursor_state.get() == "stopped"}
                    ></span>
                </h1>
                <div class="home-github" class:visible={move || show_github.get()}>
                    <a href="https://github.com/skharchikov" target="_blank" rel="noopener noreferrer" class="github-link">
                        <svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" height="24" width="24" xmlns="http://www.w3.org/2000/svg">
                            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                        </svg>
                    </a>
                </div>
            </div>
        </div>
    }
}

