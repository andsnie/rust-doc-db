use rust_doc_db::doc_db::{
    delete_entity_from_db, get_entry_from_db, insert_entity_to_db, set_entity_field_value,
    tag_entity,
};
use serde_json::{json, Value};
use serial_test::serial;
use std::fs;

use crate::test_helpers::{get_test_config, setup_test};

mod test_helpers;

#[serial]
#[test]
fn should_store_entity_in_sqlite_and_yaml() {
    setup_test();
    let db_config = get_test_config();

    let json_entity = json!({
      "title": "My day",
      "description": "Today I went to the forest",
      "date": "2021-01-01"
    });
    let entity_insert_result = insert_entity_to_db(&json_entity, &db_config).unwrap();
    assert!(!&entity_insert_result.is_nil());

    let entry_from_db = get_entry_from_db(&entity_insert_result, &db_config);
    assert!(&entry_from_db.is_ok());

    let expected_file_path = format!("{}{}.yaml", db_config.text_db_path, entity_insert_result);
    let yaml_entity = fs::read_to_string(expected_file_path).unwrap();
    assert_eq!(
        yaml_entity,
        "date: 2021-01-01\ndescription: Today I went to the forest\ntitle: My day\n"
    );
}

#[serial]
#[test]
fn can_apply_attributes_to_existing_entities() {
    setup_test();
    let db_config = get_test_config();

    let json_entity = json!({
      "title": "My day",
      "description": "Today I went to the forest",
      "date": "2021-01-01"
    });
    let entity_id = insert_entity_to_db(&json_entity, &db_config).unwrap();
    set_entity_field_value(&entity_id, "relationship", "unknown", &db_config).unwrap();

    let entry_from_db = get_entry_from_db(&entity_id, &db_config).unwrap().unwrap();
    assert_eq!(
        entry_from_db
            .entity
            .get("relationship")
            .unwrap()
            .as_str()
            .unwrap(),
        "unknown"
    );
}

#[serial]
#[test]
fn can_apply_tags_to_existing_entities() {
    setup_test();
    let db_config = get_test_config();

    let json_entity = json!({
      "title": "My day",
      "description": "Today I went to the forest",
      "date": "2021-01-01"
    });
    let entity_id = insert_entity_to_db(&json_entity, &db_config).unwrap();
    tag_entity(&entity_id, "known", &db_config).unwrap();

    let entry_from_db = get_entry_from_db(&entity_id, &db_config).unwrap().unwrap();
    let tags_from_db = entry_from_db
        .entity
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().collect())
        .unwrap_or_else(Vec::new);
    assert!(tags_from_db.contains(&&Value::String("known".to_string())));
}

#[serial]
#[test]
fn can_delete_entity() {
    setup_test();
    let db_config = get_test_config();

    let json_entity = json!({
      "title": "My day",
      "description": "Today I went to the forest",
      "date": "2021-01-01"
    });
    let entity_id = insert_entity_to_db(&json_entity, &db_config).unwrap();
    let entry_from_db = get_entry_from_db(&entity_id, &db_config).unwrap().unwrap();
    assert_eq!(
        entry_from_db.entity.get("title").unwrap().as_str().unwrap(),
        "My day"
    );

    delete_entity_from_db(&entity_id, &db_config).unwrap();
    let entry_from_db_after_deletion_result = get_entry_from_db(&entity_id, &db_config);
    match entry_from_db_after_deletion_result {
        Err(err) => {
            assert!(err.to_string().contains("Entity") && err.to_string().contains("not found"))
        }
        Ok(entry_option) => {
            if entry_option.is_some() {
                panic!("Should not be able to get deleted entity")
            }
        }
    }
}
