use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Layout)]
pub(super) fn layout(props: &Props) -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <div class="grow">
                {for props.children.iter()}
            </div>
        </div>
    }
}
