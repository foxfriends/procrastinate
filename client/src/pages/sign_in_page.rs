use crate::hooks::use_web3;
use yew::prelude::*;

#[function_component(SignInPage)]
pub(crate) fn sign_in_page() -> Html {
    let web3 = use_web3();

    eprintln!("Web3: {:?}", web3);

    html! {
        <button>
            {"Sign in"}
        </button>
    }
}
