use super::RailColor;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub color: RailColor,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Ring)]
pub(crate) fn ring(props: &Props) -> Html {
    html! {
        <div class={classes!("flex", "items-center", "justify-center", "w-ring", "h-ring", "ml-[-4px]", "border", "border-rail", "rounded-full", props.color.as_border(), props.class.clone())}>
            <div class={classes!("w-rail", "h-rail", "rounded-full", props.color.as_bg())} />
        </div>
    }
}
