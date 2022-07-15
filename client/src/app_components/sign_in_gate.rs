use crate::components::{Branch, DisplayButton, Rail, RailColor, Ring, Stop, Track, Tracks};
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
                <Tracks>
                    <Track>
                        <Rail class="w-[200px]" color={RailColor::Red} />
                        <Stop color={RailColor::Red} />
                        <div class="ml-4 font-display text-red/90">
                            {"A web3 enabled browser is required"}
                        </div>
                    </Track>
                </Tracks>
            }
        }
        Some(accounts) if accounts.is_empty() => {
            let connect_account = Callback::from(move |_| {
                let accounts = accounts.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // TODO: toast these things
                    if let Err(error) = accounts.connect().await {
                        console::error!("Error: {}", error.to_string());
                    }
                });
            });
            html! {
                <Tracks>
                    <Track>
                        <Rail class="w-[200px]" color={RailColor::RedToCyan} />
                        <Ring color={RailColor::Cyan} />
                        <DisplayButton class="ml-4" onclick={connect_account}>
                            {"Connect Account"}
                        </DisplayButton>
                    </Track>
                </Tracks>
            }
        }
        Some(accounts) => {
            let account = accounts.primary().unwrap();
            html! {
                <Tracks>
                    <Track>
                        <Rail class="w-[200px]" color={RailColor::RedToCyan} />
                        <Rail class="w-[100px]" color={RailColor::Cyan} />
                        <Stop color={RailColor::Cyan} />
                        <div class="ml-4 font-display text-lg text-cyan/90">
                            {"Connected as: "}
                            <span class="font-mono font-semibold">{account.obscured()}</span>
                        </div>
                    </Track>
                    <Track>
                        <Branch width={100} class="ml-[200px]" color={RailColor::Cyan} />
                        <Rail class="w-[100px]" color={RailColor::Cyan} />
                        <Ring color={RailColor::Cyan} />
                        <DisplayButton class="ml-4">
                            {"Sign in"}
                        </DisplayButton>
                    </Track>
                </Tracks>
            }
        }
    }
}
