use leptos::*;
use web_sys::window;

#[component]
pub fn XmasTree() -> impl IntoView {
    // Check if dark mode is enabled
    let initial_dark_mode = window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item("darkMode").ok().flatten())
        .map(|v| v == "true")
        .unwrap_or(false);

    let (dark_mode, set_dark_mode) = create_signal(initial_dark_mode);

    // Watch for dark mode changes with a simple interval check
    create_effect(move |_| {
        spawn_local(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(500).await;
                if let Some(win) = window() {
                    if let Some(doc) = win.document() {
                        if let Some(body) = doc.body() {
                            let has_dark = body.class_list().contains("dark-mode");
                            set_dark_mode.set(has_dark);
                        }
                    }
                }
            }
        });
    });

    // Build tree HTML with proper spacing
    let tree_html = move || {
        let light_class = if dark_mode.get() { "light lit" } else { "light" };
        format!(
            "&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;'.'.'.\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;-= <span class=\"{}\">o</span> =-\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;.'.'.'\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;,\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;/ \\\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;.'. <span class=\"{}\">o</span>'.\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;/ <span class=\"{}\">6</span> <span class=\"{}\">s</span> <span class=\"{}\">^</span>.\\\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;/.-.<span class=\"{}\">o</span> <span class=\"{}\">*</span>.-.\\\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`/. '.'.<span class=\"{}\">9</span>  \\`\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;'.<span class=\"{}\">6</span>. <span class=\"{}\">*</span>  <span class=\"{}\">s</span> <span class=\"{}\">o</span> '.\n\
             &nbsp;&nbsp;&nbsp;&nbsp;/.--.<span class=\"{}\">s</span> .<span class=\"{}\">6</span> .--.\\\n\
             &nbsp;&nbsp;&nbsp;&nbsp;`/ <span class=\"{}\">s</span> '. .' <span class=\"{}\">*</span> .\\`\n\
             &nbsp;&nbsp;&nbsp;.' <span class=\"{}\">o</span> <span class=\"{}\">6</span> .` .<span class=\"{}\">^</span> <span class=\"{}\">6</span> <span class=\"{}\">s</span>'.\n\
             &nbsp;&nbsp;/.---. <span class=\"{}\">*</span> <span class=\"{}\">^</span> <span class=\"{}\">o</span> .----.\\\n\
             &nbsp;&nbsp;`/<span class=\"{}\">s</span> <span class=\"{}\">*</span> `.<span class=\"{}\">^</span> <span class=\"{}\">s</span>.' <span class=\"{}\">^</span> <span class=\"{}\">*</span> \\`\n\
             &nbsp;.' <span class=\"{}\">o</span> , <span class=\"{}\">6</span> `.' <span class=\"{}\">^</span> <span class=\"{}\">o</span>  <span class=\"{}\">6</span> '.\n\
             /,-<span class=\"{}\">^</span>--,  <span class=\"{}\">o</span> <span class=\"{}\">^</span> <span class=\"{}\">*</span> <span class=\"{}\">s</span> ,----,\\\n\
             `'-._<span class=\"{}\">s</span>.;-,_<span class=\"{}\">6</span>_<span class=\"{}\">^</span>,-;._<span class=\"{}\">o</span>.-'\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;---|   |\n\
             &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`\"\"\"`",
            light_class, light_class, light_class, light_class, light_class,
            light_class, light_class, light_class, light_class, light_class,
            light_class, light_class, light_class, light_class, light_class,
            light_class, light_class, light_class, light_class, light_class,
            light_class, light_class, light_class, light_class, light_class,
            light_class, light_class, light_class, light_class, light_class,
            light_class, light_class, light_class, light_class, light_class,
            light_class, light_class, light_class, light_class, light_class,
            light_class, light_class, light_class, light_class,
        )
    };

    view! {
        <div class="xmas-tree">
            <pre class="tree-art" inner_html={move || tree_html()}></pre>
        </div>
    }
}
