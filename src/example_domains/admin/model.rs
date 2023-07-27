use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityMeta {
    pub tags: Vec<String>,
}
