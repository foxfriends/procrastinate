use yew::prelude::*;
use yew_router::prelude::*;

mod index;
mod layout;

use index::Index;
use layout::Layout;

#[derive(Clone, PartialEq, Routable, Debug)]
pub(crate) enum Route {
    // Ultimate SPA - everything is the app. Someday this may change...
    #[at("/")]
    #[not_found]
    Index,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Index => html! { <Index /> },
    }
}

#[function_component(Router)]
pub(crate) fn router() -> Html {
    html! {
        <Layout>
            <Switch<Route> render={Switch::render(switch)} />
        </Layout>
    }
}
