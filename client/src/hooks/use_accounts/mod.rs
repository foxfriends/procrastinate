use super::use_web3::{use_web3, Web3};
use futures::stream::StreamExt;
use gloo::console;
use yew::prelude::*;

mod account;
mod accounts;

pub(crate) use account::Account;
pub(crate) use accounts::Accounts;

pub(crate) fn use_accounts() -> Option<Accounts> {
    use_context::<Option<Accounts>>().unwrap()
}

pub(crate) fn use_account() -> Option<Account> {
    use_context::<Option<Accounts>>()
        .unwrap()
        .and_then(|accounts| accounts.primary())
}

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub children: Children,
}

#[function_component(AccountsProvider)]
pub(crate) fn accounts_provider(props: &Props) -> Html {
    let web3 = use_web3();
    let accounts = use_state({
        let web3 = web3.clone();
        move || web3.map(Accounts::empty)
    });

    use_effect_with_deps(
        {
            let accounts = accounts.clone();
            move |web3: &Option<Web3>| match web3.clone() {
                None => || (),
                Some(web3) => {
                    wasm_bindgen_futures::spawn_local({
                        let web3 = web3.clone();
                        let accounts = accounts.clone();
                        async move {
                            match web3.eth().accounts().await {
                                Ok(addresses) => {
                                    accounts.set(Some(Accounts::new(addresses, web3)));
                                }
                                Err(error) => console::error!("Error: {}", error.to_string()),
                            }
                        }
                    });

                    wasm_bindgen_futures::spawn_local(async move {
                        web3.transport()
                            .accounts_changed_stream()
                            .for_each(move |addresses| {
                                accounts.set(Some(Accounts::new(addresses, web3.clone())));
                                std::future::ready(())
                            })
                            .await
                    });
                    || ()
                }
            }
        },
        web3,
    );

    html! {
        <ContextProvider<Option<Accounts>> context={(*accounts).clone()}>
            {for props.children.iter()}
        </ContextProvider<Option<Accounts>>>
    }
}
