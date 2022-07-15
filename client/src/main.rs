use yew::prelude::*;
use yew_router::prelude::*;

mod app_components;
mod components;
mod hooks;
mod pages;
mod routes;

use hooks::use_accounts::AccountsProvider;
use hooks::use_web3::Web3Provider;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Web3Provider>
            <AccountsProvider>
                <BrowserRouter>
                    <routes::Router />
                </BrowserRouter>
            </AccountsProvider>
        </Web3Provider>
    }
}

fn main() {
    let app_root = gloo::utils::document()
        .query_selector("#app")
        .unwrap()
        .expect("Must be run on a page with `#app`");
    yew::start_app_in_element::<App>(app_root);
}
