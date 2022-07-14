use std::rc::Rc;
use web3::transports::eip_1193::{Eip1193, Provider};
use web3::Web3;
use yew::prelude::*;

#[derive(Clone, Debug)]
struct Web3Ref(Rc<Web3<Eip1193>>);

impl PartialEq for Web3Ref {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

pub(crate) fn use_web3() -> Rc<Web3<Eip1193>> {
    use_context::<Web3Ref>().unwrap().0
}

#[derive(Properties, PartialEq)]
pub(crate) struct Props {
    pub children: Children,
}

#[function_component(Web3Provider)]
pub(crate) fn web3_provider(props: &Props) -> Html {
    let web3 = Provider::default()
        .unwrap()
        .map(Eip1193::new)
        .map(Web3::new)
        .map(Rc::new)
        .map(Web3Ref);

    match web3 {
        Some(web3) => html! {
            <ContextProvider<Web3Ref> context={web3}>
                {for props.children.iter()}
            </ContextProvider<Web3Ref>>
        },
        None => html! {
            <div>
                {"A Web3 enabled browser is required. Try installing "}
                <a href="https://metamask.io/">{"Metamask"}</a>
            </div>
        },
    }
}
