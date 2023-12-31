#![allow(dead_code)] // HACK: any more sensible workaround for linting strictness ?

use std::collections::HashMap;

use self::{errors::DocDbError, file_storage::*, model::DocDbEntry, sql_storage::*};
use serde_json::Value;
use ulid::Ulid;

mod errors;
mod file_storage;
pub mod model;
mod sql_storage;

#[derive(Debug)]
pub struct DbConfig {
    pub sqlite_db_full_filename: String,
    pub text_db_path: String,
}

pub type DocDbResult<T> = std::result::Result<T, DocDbError>;

pub fn make_sure_db_exists(db_config: &DbConfig) -> DocDbResult<()> {
    log::info!("Checking if DB exists");
    create_sqlite_db_if_not_exists(db_config)?;
    create_text_db_if_not_exists(db_config)?;
    Ok(())
}

pub fn insert_entity_to_db(entity: &serde_json::Value, db_config: &DbConfig) -> DocDbResult<Ulid> {
    log::info!("Adding entity to DB");
    let ulid = insert_entity_to_sqlite(entity, db_config)?;
    store_entity_in_yaml_file(&ulid, entity, db_config)?;
    Ok(ulid)
}

pub fn get_entry_from_db(
    &entity_id: &Ulid,
    db_config: &DbConfig,
) -> DocDbResult<Option<DocDbEntry>> {
    log::info!("Obtaining entity {} from DB", entity_id);
    get_entry_from_sqlite(&entity_id, db_config)
}

pub fn update_entity_in_db(
    entity_id: &Ulid,
    entity: &serde_json::Value,
    db_config: &DbConfig,
) -> DocDbResult<()> {
    log::info!("Updating entity {} in DB", entity_id);
    let merged_entity = try_merge_entity_with_existing_version(entity, entity_id, db_config)?;
    update_entity_in_sqlite(entity_id, &merged_entity, db_config)?;
    store_entity_in_yaml_file(entity_id, &merged_entity, db_config)?;
    Ok(())
}

pub fn delete_entity_from_db(entity_id: &Ulid, db_config: &DbConfig) -> DocDbResult<()> {
    log::info!("Deleting entity {} from DB", entity_id);
    delete_entity_from_sqlite(entity_id, db_config)?;
    delete_yaml_file(entity_id, db_config)?;
    Ok(())
}

pub fn clear_db(db_config: &DbConfig) -> DocDbResult<()> {
    log::info!("Clearing DB");
    remove_all_entity_yaml_files(db_config)?;
    remove_all_entities_from_sqlite(db_config)?;
    Ok(())
}

fn try_merge_entity_with_existing_version(
    entity: &serde_json::Value,
    entity_id: &Ulid,
    db_config: &DbConfig,
) -> DocDbResult<serde_json::Value> {
    let mut merged_entity = entity.clone();
    let db_entry_option = get_entry_from_sqlite(entity_id, db_config)?;
    if let Some(db_entry) = db_entry_option {
        merge_entities(&db_entry.entity, &mut merged_entity)?;
    }
    Ok(merged_entity)
}

fn merge_entities(json_parent_entity: &Value, json_new_entity: &mut Value) -> DocDbResult<()> {
    if let Some(parent_entity) = json_parent_entity.as_object() {
        for (key, value) in parent_entity {
            if json_new_entity[key].is_null() {
                json_new_entity[key] = value.clone();
            }
        }
    }
    Ok(())
}

pub fn set_entity_field_value(
    entity_id: &Ulid,
    field_name: &str,
    field_value: &str,
    db_config: &DbConfig,
) -> DocDbResult<()> {
    log::info!(
        "Setting field \"{}\" value to \"{}\" for entity {}",
        field_name,
        field_value,
        entity_id
    );

    let db_entry_option = get_entry_from_db(entity_id, db_config)?;
    if db_entry_option.is_none() {
        return Err(DocDbError::SqlStorage {
            message: format!(
                "Unable to set entity {} field {} to {}",
                entity_id, field_name, field_value
            ),
            inner_type_name: "?".to_string(),
        });
    }
    let mut db_entry = db_entry_option.unwrap();
    db_entry.set_field_value(field_name, Value::String(field_value.to_string()))?;
    update_entity_in_db(entity_id, &db_entry.entity, db_config)?;
    Ok(())
}

pub fn tag_entity(entity_id: &Ulid, tag: &str, db_config: &DbConfig) -> DocDbResult<()> {
    log::info!("Adding \"{}\" tag for entity {}", tag, entity_id);

    let db_entry_option = get_entry_from_db(entity_id, db_config)?;
    if db_entry_option.is_none() {
        return Err(DocDbError::SqlStorage {
            message: format!("Unable to tag entity {}", entity_id),
            inner_type_name: "?".to_string(),
        });
    }
    let mut db_entry = db_entry_option.unwrap();

    if !db_entry.has_field("tags")? {
        log::info!("Creating tags field to store tags for entity {}", entity_id);
        let _ = db_entry.set_field_value("tags", Value::Array(Vec::new()));
    }

    let json_tags_option = db_entry.entity.get_mut("tags");
    let tags_array_option = json_tags_option
        .ok_or(DocDbError::Internal {
            message: "Unable to extract tags array".to_string(),
            inner_type_name: "?".to_string(),
        })?
        .as_array_mut();

    if let Some(tags_array) = tags_array_option {
        let tag_as_value = Value::String(String::from(tag));
        if !tags_array.contains(&tag_as_value) {
            tags_array.push(tag_as_value);
            update_entity_in_db(entity_id, &db_entry.entity, db_config)?;
        } else {
            log::info!("Tag {} already set for entity {}", tag, entity_id);
        }
    }
    Ok(())
}

pub fn untag_entity(entity_id: &Ulid, tag: &str, db_config: &DbConfig) -> DocDbResult<()> {
    log::info!("Removing \"{}\" tag for entity {}", tag, entity_id);

    let db_entry_option = get_entry_from_db(entity_id, db_config)?;
    if db_entry_option.is_none() {
        return Err(DocDbError::SqlStorage {
            message: format!("Unable to untag entity {}", entity_id),
            inner_type_name: "?".to_string(),
        });
    }
    let mut db_entry = db_entry_option.unwrap();

    if !db_entry.has_field("tags")? {
        return Ok(());
    }

    let json_tags_option = db_entry.entity.get_mut("tags");
    let tags_array_option = json_tags_option
        .ok_or(DocDbError::Internal {
            message: "Unable to extract tags array".to_string(),
            inner_type_name: "?".to_string(),
        })?
        .as_array_mut();

    if let Some(json_tags_array) = tags_array_option {
        let tags_array = json_array_to_array(json_tags_array);
        // let tag_as_value = Value::String(String::from(tag));
        if tags_array.contains(&tag) {
            let index_result = tags_array.iter().position(|x| *x == tag);
            if let Some(index) = index_result {
                json_tags_array.remove(index);
                update_entity_in_db(entity_id, &db_entry.entity, db_config)?;
            }
        }
    }
    Ok(())
}

fn json_array_to_array(json_array: &Vec<Value>) -> Vec<&str> {
    let mut strings: Vec<&str> = Vec::new();
    for x in json_array {
        if let Value::String(s) = x {
            strings.push(s);
        }
    }
    strings
}

pub fn get_entries_from_db(
    where_clause: &str,
    where_clause_params: HashMap<&str, &str>,
    db_config: &DbConfig,
) -> DocDbResult<Vec<DocDbEntry>> {
    log::info!("Querying DB: {}", where_clause);
    get_entries_from_sqlite(where_clause, where_clause_params, db_config)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::doc_db::merge_entities;

    #[test]
    pub fn test_merge_entities() {
        let old_entity = json!({
            "firstname": "John",
            "lastname": "Smith",
            "tags": ["tag1", "tag2"],
        });
        let mut new_entity = json!({
            "firstname": "Mark",
            "lastname": "Smith",
        });
        let _ = merge_entities(&old_entity, &mut new_entity);
        let expected_entity = json!({
                "firstname": "Mark",
                "lastname": "Smith",
                "tags": ["tag1", "tag2"],
        });
        assert_eq!(new_entity, expected_entity);
    }
}
