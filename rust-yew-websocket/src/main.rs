mod component;
mod services;
use yew::functional::*;
use yew::prelude::*;

use crate::component::{
    login::Login,
    chat::Chat
};


fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
            <div class="flex w-screen h-screen">
                <Login/>
            </div>
    }
}
