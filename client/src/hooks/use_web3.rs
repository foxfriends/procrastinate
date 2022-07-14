use std::rc::Rc;
use web3::transports::eip_1193::{Eip1193, Provider};
use yew::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct Web3(Rc<web3::Web3<Eip1193>>);

impl PartialEq for Web3 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl std::ops::Deref for Web3 {
    type Target = web3::Web3<Eip1193>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

pub(crate) fn use_web3() -> Option<Web3> {
    use_context::<Option<Web3>>().unwrap()
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
        .map(web3::Web3::new)
        .map(Rc::new)
        .map(Web3);

    html! {
        <ContextProvider<Option<Web3>> context={web3}>
            {for props.children.iter()}
        </ContextProvider<Option<Web3>>>
    }
}
