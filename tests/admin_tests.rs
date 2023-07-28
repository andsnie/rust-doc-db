use rust_doc_db::{
    doc_db::{get_entry_from_db, insert_entity_to_db},
    example_domains::{
        admin::{mark_entity_as_important, model::EntityMeta, unmark_entity_as_important},
        pim::fake_data_generator,
    },
};
use serde_json::json;
use serial_test::serial;

use crate::test_helpers::{get_test_config, setup_test};

mod test_helpers;

#[serial]
#[test]
fn admin_can_tag_untag_entities() {
    setup_test();
    let db_config = get_test_config();

    let people = fake_data_generator::generate_people(1);
    let person = people.first();
    let json_person = json!(person);

    let entity_id = insert_entity_to_db(&json_person, &db_config).unwrap();
    mark_entity_as_important(&entity_id, &db_config).unwrap();

    let db_entry_1 = get_entry_from_db(&entity_id, &db_config).unwrap();
    let entity_as_meta_1: EntityMeta = serde_json::from_value(db_entry_1.entity).unwrap();
    assert!(entity_as_meta_1.tags.contains(&"important".to_string()));

    unmark_entity_as_important(&entity_id, &db_config).unwrap();

    let db_entry_2 = get_entry_from_db(&entity_id, &db_config).unwrap();
    let entity_as_meta_2: EntityMeta = serde_json::from_value(db_entry_2.entity).unwrap();
    assert!(!entity_as_meta_2.tags.contains(&"important".to_string()));
}
