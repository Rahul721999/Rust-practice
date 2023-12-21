use crate::Chat;
use log::debug;
use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let logged_in = use_state(|| false);

    // handle onUsernameInput event
    let handle_username_input = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };
    // handle onPasswordInput event
    let handle_password_input = {
        let current_password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_password.set(input.value());
        })
    };

    // handle onSubmit event
    let handle_submit_event = {
        let username = username.clone();
        let password = password.clone();
        let logged_in = logged_in.clone();
        Callback::from(move |_| {
            logged_in.set(true);
            debug!("username: {},Password: {}", *username, *password);
        })
    };

    let login = html! {
        <div>
            <form onsubmit={handle_submit_event}>
                <input
                    oninput = {handle_username_input}
                    placeholder="Username"
                />
                <input
                    oninput = {handle_password_input}
                    placeholder="password"
                />
                <button type="submit">{"Submit"}
                </button>
            </form>
        </div>
    };
    if *logged_in{
        html! {<Chat/>}
    } 
    else{login}
}
