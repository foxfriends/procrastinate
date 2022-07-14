use crate::hooks::use_accounts;
use gloo::console;
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub(crate) struct Props {
    pub children: Children,
}

#[function_component(SignInGate)]
pub(crate) fn sign_in_page(props: &Props) -> Html {
    let accounts = use_accounts();

    match accounts {
        None => {
            html! {
                "A web3 enabled browser is required"
            }
        }
        Some(accounts) if accounts.is_empty() => {
            let connect_accounts = move |_| {
                let accounts = accounts.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // TODO: toast these things
                    if let Err(error) = accounts.connect().await {
                        console::error!("Error: {}", error.to_string());
                    }
                });
            };
            html! {
                <button onclick={connect_accounts}>
                    {"Connect Accounts"}
                </button>
            }
        }
        Some(..) => {
            html! {
                {for props.children.iter()}
            }
        }
    }
}
