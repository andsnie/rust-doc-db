use serde_json::Value;
use ulid::Ulid;

use super::{errors::DocDbError, DocDbResult};

#[derive(Debug)]
pub struct DocDbEntry {
    pub id: Ulid,
    pub entity: Value,
}

impl DocDbEntry {
    pub fn set_field_value(&mut self, field_name: &str, field_value: Value) -> DocDbResult<()> {
        self.entity
            .as_object_mut()
            .ok_or(DocDbError::InternalError {
                message: "Unable to extract tags array".to_string(),
                inner_type_name: "?".to_string(),
            })?
            .insert(field_name.to_string(), field_value);
        return Ok(());
    }

    pub fn has_field(&self, field_name: &str) -> DocDbResult<bool> {
        let result = self
            .entity
            .as_object()
            .ok_or(DocDbError::InternalError {
                message: "Unable to extract tags array".to_string(),
                inner_type_name: "?".to_string(),
            })?
            .contains_key(field_name);
        return Ok(result);
    }
}
