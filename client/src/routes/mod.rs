use yew::prelude::*;
use yew_router::prelude::*;

mod layout;
use layout::Layout;

mod app;
mod index;

use app::App;
use index::Index;

#[derive(Clone, PartialEq, Routable, Debug)]
pub(crate) enum Route {
    #[at("/")]
    #[not_found]
    Index,
    #[at("/app")]
    App,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Index => html! { <Index /> },
        Route::App => html! { <App /> },
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
