use yew::prelude::*;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col min-h-screen">
                <div class="grow">
                    {"Hello world"}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
