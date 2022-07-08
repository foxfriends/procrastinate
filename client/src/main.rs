use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <div class="grow">
                {"Hello world"}
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
