mod app;
mod components;
mod data;
mod utils;
mod views;

use app::*;

fn main() {
    leptos::mount::mount_to_body(App)
}
