#![allow(dead_code)]

pub mod model;

use ulid::Ulid;

use crate::doc_db::{tag_entity, untag_entity, DbConfig, DocDbResult};

pub fn mark_entity_as_important(entity_id: &Ulid, db_config: &DbConfig) -> DocDbResult<()> {
    tag_entity(entity_id, "important", &db_config)?;
    return Ok(());
}

pub fn unmark_entity_as_important(entity_id: &Ulid, db_config: &DbConfig) -> DocDbResult<()> {
    untag_entity(entity_id, "important", &db_config)?;
    return Ok(());
}
