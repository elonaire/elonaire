mod app;
mod views;
mod components;
mod data;
mod utils;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
