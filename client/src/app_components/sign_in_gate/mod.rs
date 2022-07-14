use crate::hooks::use_accounts;
use gloo::console;
use yew::prelude::*;

mod rail;

use rail::Rail;

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
                <Rail line="bg-red" end="bg-red">
                    <div class="font-display text-lg text-red/90">
                        {"A web3 enabled browser is required"}
                    </div>
                </Rail>
            }
        }
        Some(accounts) if accounts.is_empty() => {
            let connect_account = move |_| {
                let accounts = accounts.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // TODO: toast these things
                    if let Err(error) = accounts.connect().await {
                        console::error!("Error: {}", error.to_string());
                    }
                });
            };
            html! {
                <Rail line="bg-gradient-to-r from-red via-yellow to-cyan" end="bg-cyan">
                    <button class="font-display text-lg text-cyan/90 hover:text-white" onclick={connect_account}>
                        {"Connect Account"}
                    </button>
                </Rail>
            }
        }
        Some(..) => {
            html! {
                {for props.children.iter()}
            }
        }
    }
}
