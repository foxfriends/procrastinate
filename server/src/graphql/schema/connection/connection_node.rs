use super::Cursor;

pub(crate) trait ConnectionNode {
    fn cursor(&self) -> Cursor;
}
