use crate::components::Programmer;
use leptos::*;

/// True when the user has requested reduced motion at the OS level.
fn prefers_reduced_motion() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
        .map(|mql| mql.matches())
        .unwrap_or(false)
}

#[component]
pub fn Home() -> impl IntoView {
    let command = "whoami";
    let full_name = "skharchikov";

    let (typed_cmd, set_typed_cmd) = create_signal(String::new());
    let (typed_name, set_typed_name) = create_signal(String::new());
    // "cmd" = typing the command, "name" = output printed (command swapped out)
    let (phase, set_phase) = create_signal("cmd");
    let (show_extras, set_show_extras) = create_signal(false);

    create_effect(move |_| {
        // Honor reduced-motion: render the final state immediately, no typing.
        if prefers_reduced_motion() {
            set_typed_name.set(full_name.to_string());
            set_phase.set("name");
            set_show_extras.set(true);
            return;
        }

        let cmd_chars: Vec<char> = command.chars().collect();

        spawn_local(async move {
            // Pause before the prompt starts typing.
            gloo_timers::future::TimeoutFuture::new(400).await;

            // Phase 1: type `whoami` on the hero line.
            for ch in cmd_chars.iter() {
                let delay = 90 + (js_sys::Math::random() * 110.0) as i32;
                gloo_timers::future::TimeoutFuture::new(delay as u32).await;
                set_typed_cmd.update(|s| s.push(*ch));
            }

            // "Press enter": brief pause, then the same line swaps to the output.
            gloo_timers::future::TimeoutFuture::new(800).await;
            set_typed_name.set(full_name.to_string());
            set_phase.set("name");

            // Reveal the gif's friend tagline shortly after.
            gloo_timers::future::TimeoutFuture::new(350).await;
            set_show_extras.set(true);
        });
    });

    view! {
        <div class="home-container">
            <div class="home-content">
                <div class="home-hero">
                    <Programmer visible=Signal::derive(|| true) />
                    <h1 class="home-logo">
                        {move || if phase.get() == "cmd" {
                            view! { <span class="prompt">"~ $ "</span>{move || typed_cmd.get()} }.into_view()
                        } else {
                            view! { {move || typed_name.get()} }.into_view()
                        }}
                        <span class="cursor gradient-caret"></span>
                    </h1>
                </div>
                <div class="home-tagline" class:visible={move || show_extras.get()}>
                    <span class="tagline-skills"><code class="tagline-cmd">"cargo build"</code>"-ing boring-reliable backends in "<b>"Rust"</b>" & "<b>"Scala"</b></span>
                </div>
            </div>
        </div>
    }
}
