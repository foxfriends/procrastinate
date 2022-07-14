use yew::prelude::*;

use crate::app_components::{MainNav, SignInGate};

#[function_component(AppPage)]
pub(crate) fn app_page() -> Html {
    html! {
        <SignInGate>
            <MainNav />
        </SignInGate>
    }
}
