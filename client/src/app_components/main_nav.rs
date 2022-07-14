use crate::hooks::use_account;
use yew::prelude::*;

#[function_component(MainNav)]
pub(crate) fn main_nav() -> Html {
    let account = use_account().unwrap();

    html! {
        <div>
            {"Connected with: "}{account.checksum()}
        </div>
    }
}
