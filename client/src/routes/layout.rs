use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Layout)]
pub(super) fn layout(props: &Props) -> Html {
    html! {
        <div class="flex flex-col min-h-screen bg-black text-white font-body">
            <div class="flex flex-col grow">
                {for props.children.iter()}
            </div>
        </div>
    }
}
