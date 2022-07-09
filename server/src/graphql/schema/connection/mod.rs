mod connection_node;
mod connection_result;
mod connector;
mod cursor;
mod edge;
mod page_info;

pub(crate) use connection_node::ConnectionNode;
pub(crate) use connection_result::connection;
pub(crate) use connection_result::ConnectionResult;
pub(crate) use connector::{Connector, DatabaseConnector};
pub(crate) use cursor::Cursor;
pub(crate) use edge::connection_edge;
pub(crate) use edge::Edge;
pub(crate) use page_info::PageInfo;

pub(crate) struct Connection<T: ConnectionNode> {
    edges: Vec<Edge<T>>,
    page_info: PageInfo,
}

impl<T: ConnectionNode> Connection<T> {
    pub fn new(edges: Vec<Edge<T>>, page_info: PageInfo) -> Self {
        Self { edges, page_info }
    }

    pub fn edges(&self) -> &[Edge<T>] {
        &self.edges
    }

    pub fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    pub fn empty_end(has_previous_page: bool) -> Self {
        Self {
            edges: vec![],
            page_info: PageInfo::empty_end(has_previous_page),
        }
    }

    pub fn empty_start(has_next_page: bool) -> Self {
        Self {
            edges: vec![],
            page_info: PageInfo::empty_start(has_next_page),
        }
    }
}
