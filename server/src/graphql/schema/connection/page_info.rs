use super::Cursor;

#[derive(Clone, Debug, juniper::GraphQLObject)]
pub(crate) struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
    pub start_cursor: Cursor,
    pub end_cursor: Cursor,
}

impl PageInfo {
    pub fn empty_start(has_next_page: bool) -> Self {
        Self {
            has_previous_page: false,
            has_next_page,
            start_cursor: Cursor::Start,
            end_cursor: Cursor::Start,
        }
    }

    pub fn empty_end(has_previous_page: bool) -> Self {
        Self {
            has_previous_page,
            has_next_page: false,
            start_cursor: Cursor::End,
            end_cursor: Cursor::End,
        }
    }
}
