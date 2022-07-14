use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(super) struct Props {
    pub line: Classes,
    pub end: Classes,
    pub children: Children,
}

#[function_component(Rail)]
pub(super) fn rail(props: &Props) -> Html {
    html! {
        <div class="flex items-center">
            <div class={classes!("w-[200px]", "h-rail", props.line.clone())} />
            <div class={classes!("w-[100px]", "h-rail", props.end.clone())} />
            <div class={classes!("w-rail", "h-stop", "rounded-full", props.end.clone())} />
            <div class="ml-4">
                {for props.children.iter()}
            </div>
        </div>
    }
}
