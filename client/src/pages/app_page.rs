use yew::prelude::*;

use crate::components::SignInGate;

#[function_component(AppPage)]
pub(crate) fn app_page() -> Html {
    html! {
        <SignInGate>
            {"Welcome to the app"}
        </SignInGate>
    }
}
