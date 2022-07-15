use crate::components::{Rail, RailColor, Ring, Stop};
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
                <div class="flex items-center">
                    <Rail class="w-[200px]" color={RailColor::red()} />
                    <Stop color={RailColor::red()} />
                    <div class="ml-4 font-display text-red/90">
                        {"A web3 enabled browser is required"}
                    </div>
                </div>
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
                <div class="flex items-center">
                    <Rail class="w-[200px]" color={RailColor::red_to_cyan()} />
                    <Ring color={RailColor::cyan()} />
                    <button class="ml-4 font-display text-lg text-cyan/90 hover:text-white" onclick={connect_account}>
                        {"Connect Account"}
                    </button>
                </div>
            }
        }
        Some(..) => {
            html! {
                {for props.children.iter()}
            }
        }
    }
}
