use crate::components::{Branch, DisplayButton, Rail, RailColor, Ring, Stop, Track, Tracks};
use crate::hooks::{use_accounts, use_api, use_query};
use gloo::console;
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub(crate) struct Props {
    pub children: Children,
}

#[function_component(SignInGate)]
pub(crate) fn sign_in_gate(props: &Props) -> Html {
    let accounts = use_accounts();
    let api = use_api();
    let authorized = use_query(|api| async move { api.check_auth().await }, api);
    let is_authorized = authorized.data().as_deref().copied().unwrap_or(false);

    match accounts {
        None => html! {
            <Tracks>
                <Track>
                    <Rail class="w-[200px]" color={RailColor::Red} />
                    <Stop color={RailColor::Red} />
                    <div class="ml-4 font-display text-red/90">
                        {"A web3 enabled browser is required"}
                    </div>
                </Track>
            </Tracks>
        },
        Some(accounts) => match accounts.primary() {
            None => {
                let connect_account = Callback::from(move |_| {
                    let accounts = accounts.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        // TODO: toast these things
                        if let Err(error) = accounts.connect().await {
                            console::error!(error.to_string());
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
            Some(account) if !is_authorized => {
                let api = use_api();
                let sign_in = Callback::from(move |_| {
                    let api = api.clone();
                    let authorized = authorized.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Err(error) = api.sign_in().await {
                            console::error!(error.to_string());
                        }
                        authorized.set(From::from(true));
                    })
                });
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
                            <DisplayButton class="ml-4" onclick={sign_in}>
                                {"Sign in"}
                            </DisplayButton>
                        </Track>
                    </Tracks>
                }
            }
            Some(account) => html! {
                <Tracks>
                    <Track>
                        <Rail class="w-[150px]" color={RailColor::Yellow} />
                        <Stop color={RailColor::Yellow} />
                        <div class="ml-4 font-display text-lg text-yellow/90">
                            {"Connected as: "}
                            <span class="font-mono font-semibold">{account.obscured()}</span>
                        </div>
                    </Track>
                    {for props.children.iter()}
                </Tracks>
            },
        },
    }
}
