use leptos::*;

#[component]
pub fn Programmer(
    #[prop(into)] visible: Signal<bool>
) -> impl IntoView {
    view! {
        <div class="programmer-animation" class:visible={move || visible.get()}>
            <img src="/programmer.gif" alt="Programmer animation" class="programmer-gif" />
        </div>
    }
}
