use super::{Attachment, Id, Reaction, RichText};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    id: Id,
    content: RichText,
    attachments: Vec<Attachment>,
    reactions: Vec<Reaction>,
    replies: Vec<Message>,
}
