use super::RailColor;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub color: RailColor,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Stop)]
pub(crate) fn stop(props: &Props) -> Html {
    html! {
        <div
            class={classes!("w-rail", "h-stop", "rounded-full", props.color.as_bg(), props.class.clone())}
        />
    }
}
