use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Track)]
pub(crate) fn track(props: &Props) -> Html {
    html! {
        <div class={classes!("flex", "items-center", "h-stop", props.class.clone())}>
            {for props.children.iter()}
        </div>
    }
}
