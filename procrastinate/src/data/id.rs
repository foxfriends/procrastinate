use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Id(Uuid);
