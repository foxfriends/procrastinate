use std::future::Future;
use std::rc::Rc;
use yew::prelude::*;

#[derive(PartialEq, Debug)]
pub(crate) struct QueryResult<T, E> {
    data: Option<Rc<T>>,
    error: Option<Rc<E>>,
    is_loading: bool,
}

impl<T, E> Default for QueryResult<T, E> {
    fn default() -> Self {
        Self {
            data: None,
            error: None,
            is_loading: false,
        }
    }
}

impl<T, E> Clone for QueryResult<T, E> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            error: self.error.clone(),
            is_loading: self.is_loading,
        }
    }
}

#[allow(dead_code)]
impl<T, E> QueryResult<T, E> {
    fn start_loading(&self) -> Self {
        Self {
            is_loading: true,
            ..self.clone()
        }
    }

    pub fn result(&self) -> Option<Result<Rc<T>, Rc<E>>> {
        match self {
            Self {
                data: Some(data), ..
            } => Some(Ok(data.clone())),
            Self {
                error: Some(error), ..
            } => Some(Err(error.clone())),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<Rc<T>> {
        self.data.clone()
    }

    pub fn error(&self) -> Option<Rc<E>> {
        self.error.clone()
    }

    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }

    pub fn is_ok(&self) -> bool {
        self.data.is_some()
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }
}

impl<T, E> From<Result<T, E>> for QueryResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        let (data, error) = match result {
            Ok(data) => (Some(Rc::new(data)), None),
            Err(error) => (None, Some(Rc::new(error))),
        };
        Self {
            data,
            error,
            is_loading: false,
        }
    }
}

impl<T, E> From<T> for QueryResult<T, E> {
    fn from(data: T) -> Self {
        Self {
            data: Some(Rc::new(data)),
            error: None,
            is_loading: false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct UseQueryOptions {
    pub enabled: bool,
}

impl Default for UseQueryOptions {
    fn default() -> Self {
        Self { enabled: true }
    }
}

pub(crate) fn use_query_with_options<T, E, F, Q, D>(
    query: Q,
    options: UseQueryOptions,
    deps: D,
) -> UseStateHandle<QueryResult<T, E>>
where
    F: Future<Output = Result<T, E>>,
    Q: FnOnce(D) -> F + 'static,
    D: Clone + PartialEq + 'static,
    T: 'static,
    E: 'static,
{
    let output = use_state(QueryResult::default);
    let deps_ref = use_mut_ref(|| None);
    use_effect_with_deps(
        {
            let output = output.clone();
            move |(deps, options): &(D, UseQueryOptions)| {
                let deps = deps.clone();
                if options.enabled {
                    let mut prev_deps = deps_ref.borrow_mut();
                    let deps_changed = prev_deps.is_none() || deps != *prev_deps.as_ref().unwrap();
                    if deps_changed {
                        let deps = deps.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            output.set(output.start_loading());
                            let result = query(deps).await;
                            output.set(QueryResult::from(result));
                        });
                    }
                    *prev_deps = Some(deps);
                } else {
                    *deps_ref.borrow_mut() = None;
                }
                || ()
            }
        },
        (deps, options),
    );
    output
}

pub(crate) fn use_query<T, E, F, Q, D>(query: Q, deps: D) -> UseStateHandle<QueryResult<T, E>>
where
    F: Future<Output = Result<T, E>>,
    Q: FnOnce(D) -> F + 'static,
    D: Clone + PartialEq + 'static,
    T: 'static,
    E: 'static,
{
    use_query_with_options(query, UseQueryOptions::default(), deps)
}
