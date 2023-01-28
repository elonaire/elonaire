mod app;
mod views;
mod components;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
