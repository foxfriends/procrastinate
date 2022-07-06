use serde::{Deserialize, Serialize};

use super::Id;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reaction {
    user: Id,
    value: String,
}
