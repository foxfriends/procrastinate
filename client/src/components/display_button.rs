use super::RailColor;
use yew::prelude::*;
use web_sys::MouseEvent;

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    #[prop_or(RailColor::Cyan)]
    pub color: RailColor,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    pub children: Children,
}

#[function_component(DisplayButton)]
pub(crate) fn display_button(props: &Props) -> Html {
    html! {
        <button
            class={classes!("font-display", "text-lg", "text-cyan/90", "hover:text-white", props.class.clone())}
            onclick={props.onclick.clone()}
        >
            {for props.children.iter()}
        </button>
    }
}
