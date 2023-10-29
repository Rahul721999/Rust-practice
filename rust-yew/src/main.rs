mod app;
mod component;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
