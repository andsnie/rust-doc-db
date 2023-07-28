#![allow(dead_code)]

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use sqlite::State;
use ulid::Ulid;

use super::{model::DocDbEntry, DbConfig, DocDbResult};
use crate::doc_db::{errors::DocDbError, DocDbError::SqlStorageError};

pub fn get_sqlite_connection(db_full_filename: &str) -> Result<sqlite::Connection, sqlite::Error> {
    return sqlite::open(db_full_filename);
}

pub fn create_sqlite_db_if_not_exists(db_config: &DbConfig) -> DocDbResult<bool> {
    log::info!(
        "Creating SQLite database in {}",
        db_config.sqlite_db_full_filename
    );
    if Path::new(&db_config.sqlite_db_full_filename).exists() {
        return Ok(true);
    }

    let mut sqlite_db_path = PathBuf::from(&db_config.sqlite_db_full_filename);
    sqlite_db_path.pop();
    fs::create_dir_all(&sqlite_db_path.to_str().ok_or(DocDbError::InternalError {
        message: format!(
            "Unable to process DB path {}",
            db_config.sqlite_db_full_filename
        ),
        inner_type_name: "?".to_string(),
    })?)?;

    let connection = get_sqlite_connection(&db_config.sqlite_db_full_filename)?;
    let mut statement = connection
    .prepare("CREATE TABLE `entities` ( `id` TEXT NOT NULL UNIQUE, `content` TEXT NOT NULL, PRIMARY KEY(`id`) )")?;
    statement.next()?;
    return Ok(true);
}

pub fn get_entry_from_sqlite(entity_id: &Ulid, db_config: &DbConfig) -> DocDbResult<DocDbEntry> {
    log::info!("Obtaining entity {} from SQLite", entity_id);
    let connection = get_sqlite_connection(&db_config.sqlite_db_full_filename)?;

    let mut statement = connection.prepare("SELECT content FROM entities WHERE id=:id")?;
    statement.bind((1, entity_id.to_string().as_str()))?;
    while let Ok(State::Row) = statement.next() {
        let raw_json = statement.read::<String, _>(0).unwrap();
        let entry = DocDbEntry {
            id: *entity_id,
            entity: serde_json::from_str(raw_json.as_str())?,
        };
        return Ok(entry);
    }
    return Err(SqlStorageError {
        message: format!("Entity {} not found", entity_id.to_string()),
        inner_type_name: "?".to_string(), // TODO:
    });
}

pub fn insert_entity_to_sqlite(
    entity: &serde_json::Value,
    db_config: &DbConfig,
) -> DocDbResult<Ulid> {
    let entity_id = Ulid::new();
    log::info!("Inserting entity {} to SQLite", entity_id);
    let connection = get_sqlite_connection(&db_config.sqlite_db_full_filename)?;

    let mut statement =
        connection.prepare("INSERT INTO entities (id, content) VALUES (:id, :content)")?;
    statement.bind((":id", entity_id.to_string().as_str()))?;
    statement.bind((":content", entity.to_string().as_str()))?;
    statement.next()?;
    return Ok(entity_id);
}

pub fn update_entity_in_sqlite(
    entity_id: &Ulid,
    entity: &serde_json::Value,
    db_config: &DbConfig,
) -> DocDbResult<()> {
    log::info!("Updating entity {} in SQLite", entity_id);
    let connection = get_sqlite_connection(&db_config.sqlite_db_full_filename)?;
    let mut statement = connection.prepare("UPDATE entities SET content=:content WHERE id=:id")?;
    statement.bind((":id", entity_id.to_string().as_str()))?;
    statement.bind((":content", entity.to_string().as_str()))?;
    statement.next()?;
    return Ok(());
}

pub fn delete_entity_from_sqlite(entity_id: &Ulid, db_config: &DbConfig) -> DocDbResult<()> {
    log::info!("Removing entity {} from SQLite", entity_id);
    let connection = get_sqlite_connection(&db_config.sqlite_db_full_filename)?;
    let mut statement = connection.prepare("DELETE FROM entities WHERE id=:id")?;
    statement.bind((":id", entity_id.to_string().as_str()))?;
    statement.next()?;
    return Ok(());
}

pub fn remove_all_entities_from_sqlite(db_config: &DbConfig) -> DocDbResult<()> {
    log::info!("Removing all entities from SQLite");
    let connection = get_sqlite_connection(&db_config.sqlite_db_full_filename)?;
    let query = format!("DELETE FROM entities");
    connection.execute(query)?;
    return Ok(());
}

pub fn get_entries_from_sqlite(
    where_clause: &str,
    where_clause_params: HashMap<&str, &str>,
    db_config: &DbConfig,
) -> DocDbResult<Vec<DocDbEntry>> {
    let connection = get_sqlite_connection(&db_config.sqlite_db_full_filename)?;
    let mut statement = connection.prepare(format!(
        "SELECT id, content FROM entities WHERE {}",
        where_clause
    ))?;
    for (key, value) in where_clause_params {
        statement.bind((format!(":{}", key).as_str(), value))?;
    }
    let mut entities: Vec<DocDbEntry> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        let entity_id = statement.read::<String, _>("id")?;
        let entity = statement.read::<String, _>("content")?;
        let json_entity = serde_json::from_str(&entity)?;
        let entry = DocDbEntry {
            id: Ulid::from_string(entity_id.as_str())?,
            entity: json_entity,
        };
        entities.push(entry);
    }
    return Ok(entities);
}
