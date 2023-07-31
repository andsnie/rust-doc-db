use rust_doc_db::{
    doc_db::{get_entry_from_db, insert_entity_to_db},
    example_domains::{
        admin::{mark_entity_as_important, model::EntityMeta},
        pim::fake_data_generator,
    },
};
use serde_json::{json, Value};
use serial_test::serial;

use crate::test_helpers::{get_test_config, setup_test};

mod test_helpers;

#[serial]
#[test]
fn top_level_fields_are_preserved_across_domains() {
    setup_test();
    let db_config = get_test_config();

    let people = fake_data_generator::generate_people(1);
    let person = people.first();
    let json_person = json!(person);

    let entity_id = insert_entity_to_db(&json_person, &db_config).unwrap();
    mark_entity_as_important(&entity_id, &db_config).unwrap();

    let db_entry = get_entry_from_db(&entity_id, &db_config).unwrap().unwrap();
    let tags_from_db = db_entry
        .entity
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().collect())
        .unwrap_or_else(Vec::new);
    assert!(tags_from_db.contains(&&Value::String("important".to_string())));

    let entity_as_meta: EntityMeta = serde_json::from_value(db_entry.entity).unwrap();
    assert!(entity_as_meta.tags.contains(&"important".to_string()));
}
