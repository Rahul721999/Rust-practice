mod component;
mod services;
use component::{chat::Chat, login::Login};
use std::cell::RefCell;
use std::rc::Rc;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;




pub type User = Rc<UserCred>;
#[derive(Debug, PartialEq)]
pub struct UserCred {
    pub username: RefCell<String>,
    pub password: RefCell<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/chat")]
    Chat,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(selected_route: &Route) -> Html {
    match selected_route {
        Route::Login => html! {<Login />},
        Route::Chat => html! {<Chat/>},
        Route::NotFound => html! {<h1>{"404 baby"}</h1>},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let ctx = use_state(|| {
        Rc::new(UserCred {
            username: RefCell::new("initial".into()),
            password: RefCell::new("initial".into()),
        })
    });

    html! {
     <ContextProvider<User> context={(*ctx).clone()}>
        // <BrowserRouter>
            <div class="flex w-screen h-screen">
                // <Switch<Route> render={Switch::render(switch)}/>
            </div>
        // </BrowserRouter>
    </ContextProvider<User>>
    }
}
