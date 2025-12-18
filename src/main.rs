use leptos::*;

mod app;
mod components;
mod models;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

    mount_to_body(App)
}
