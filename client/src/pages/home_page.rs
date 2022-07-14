use crate::app_components::Link;
use crate::routes::Route;
use yew::prelude::*;

#[function_component(HomePage)]
pub(crate) fn home_page() -> Html {
    html! {
        <div class="grow flex flex-col items-center justify-center gap-8">
            <h1 class="font-display text-2xl">
                {"This app needs a real name"}
            </h1>
            <h2 class="text-xl">
                <Link to={Route::App}>
                    {"Let's go"}
                </Link>
            </h2>
        </div>
    }
}
