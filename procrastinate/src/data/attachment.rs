use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Attachment {
    Preview(Url),
    Image(Url),
    File(Url),
}
