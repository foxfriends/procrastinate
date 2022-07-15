use super::RailColor;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub width: u32,
    pub color: RailColor,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Branch)]
pub(crate) fn branch(props: &Props) -> Html {
    html! {
        <svg
            class={classes!("relative", "shrink-0", "translate-y-[calc(-50%+3px)]", props.class.clone())}
            width={props.width.to_string()}
            height="54"
            viewBox={format!("0 0 {} 54", props.width)}
            xmlns="http://www.w3.org/2000/svg">
            <path
                d={format!("M0 3 C{} 3,{} 51,{} 51", props.width / 2, props.width / 2, props.width)}
                stroke-linecap="square"
                stroke-width="6"
                stroke={props.color.as_stroke()}
                fill="none" />
        </svg>
    }
}
