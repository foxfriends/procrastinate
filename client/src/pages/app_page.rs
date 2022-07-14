use crate::app_components::SignInGate;
use yew::prelude::*;

#[function_component(AppPage)]
pub(crate) fn app_page() -> Html {
    html! {
        <div class="flex flex-col my-4">
            <SignInGate>
                {"Hello"}
            </SignInGate>
        </div>
    }
}
