mod app;
mod components;
mod schemas;
mod utils;
mod views;

use app::*;

fn main() {
    leptos::mount::mount_to_body(App)
}
