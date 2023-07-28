#![allow(dead_code)]

use glob::glob;
use std::fs::File;
use std::io::prelude::*;
use std::{fs, path::Path};
use ulid::Ulid;

use super::{DbConfig, DocDbResult};

pub fn create_text_db_if_not_exists(db_config: &DbConfig) -> DocDbResult<()> {
    let path = Path::new(&db_config.text_db_path);
    if path.exists() {
        return Ok(());
    }
    log::info!("Creating text DB in {}", db_config.text_db_path);
    fs::create_dir_all(&db_config.text_db_path)?;
    return Ok(());
}

pub fn store_entity_in_yaml_file(
    entity_id: &Ulid,
    entity: &serde_json::Value,
    db_config: &DbConfig,
) -> DocDbResult<()> {
    let yaml_str = serde_yaml::to_string(&entity)?;
    let filename = format!("{}{}.yaml", db_config.text_db_path, entity_id.to_string());
    log::info!("Saving entity {} text DB as {}", entity_id, filename);
    let mut file = File::create(filename)?;
    file.write_all(yaml_str.as_bytes())?;
    return Ok(());
}

pub fn delete_yaml_file(entity_id: &Ulid, db_config: &DbConfig) -> DocDbResult<()> {
    let filename = format!("{}{}.yaml", db_config.text_db_path, entity_id.to_string());
    log::info!("Removing entity {} from text DB in {}", entity_id, filename);
    fs::remove_file(&filename)?;
    return Ok(());
}

pub fn remove_all_entity_yaml_files(db_config: &DbConfig) -> DocDbResult<()> {
    let filemask = format!("{}*.yaml", db_config.text_db_path);
    log::info!(
        "Removing all entity files from text DB with mask {}",
        filemask
    );
    for filename_result in glob(&filemask)? {
        let filename = filename_result?;
        log::debug!("Removing entity {}", filename.display());
        fs::remove_file(filename)?;
    }
    return Ok(());
}
