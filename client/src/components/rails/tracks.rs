use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Tracks)]
pub(crate) fn tracks(props: &Props) -> Html {
    html! {
        <div class={classes!("flex", "flex-col", props.class.clone())}>
            {for props.children.iter()}
        </div>
    }
}
