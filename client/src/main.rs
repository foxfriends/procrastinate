use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod hooks;
mod pages;
mod routes;

use hooks::use_web3::Web3Provider;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Web3Provider>
            <BrowserRouter>
                <routes::Router />
            </BrowserRouter>
        </Web3Provider>
    }
}

fn main() {
    yew::start_app::<App>();
}
