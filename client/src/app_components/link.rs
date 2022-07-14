use crate::routes::Route;
use yew::prelude::*;
use yew_router::components::Link as RouterLink;

#[derive(PartialEq, Properties)]
pub(crate) struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    pub to: Route,
    pub children: Children,
}

#[function_component(Link)]
pub(crate) fn link(props: &Props) -> Html {
    html! {
        <RouterLink<Route>
            disabled={props.disabled}
            to={props.to.clone()}
            classes={classes!("text-cyan", "hover:text-white", props.class.clone())}
        >
            {for props.children.iter()}
        </RouterLink<Route>>
    }
}
