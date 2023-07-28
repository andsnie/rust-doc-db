use rust_doc_db::{
    doc_db::insert_entity_to_db,
    example_domains::pim::{get_phones_of_people_by_firstname, model::Person},
};
use serde_json::json;
use serial_test::serial;
use test_helpers::{get_test_config, setup_test};
use ulid::Ulid;

mod test_helpers;

#[serial]
#[test]
fn can_query_phones_by_name() {
    setup_test();
    let db_config = get_test_config();

    let mut people = Vec::new();
    people.push(Person {
        firstname: "Piotr".to_string(),
        lastname: "Nowak".to_string(),
        phones: vec!["+48 123 456 789".to_string(), "+48 789 123 456".to_string()],
        addresses: Vec::new(),
    });
    people.push(Person {
        firstname: "Tomasz".to_string(),
        lastname: "Zawadzki".to_string(),
        phones: vec!["+48 123 333 444".to_string()],
        addresses: Vec::new(),
    });
    people.push(Person {
        firstname: "Piotr".to_string(),
        lastname: "Malinowski".to_string(),
        phones: vec!["+48 345 345 345".to_string()],
        addresses: Vec::new(),
    });

    let mut people_ids: Vec<Ulid> = Vec::new();
    for person in people {
        people_ids.push(insert_entity_to_db(&json!(person), &db_config).unwrap())
    }

    let phones = get_phones_of_people_by_firstname("Piotr", &db_config).unwrap();
    assert_eq!(phones.len(), 3);
    assert_eq!(phones[0], "+48 123 456 789");
    assert_eq!(phones[1], "+48 789 123 456");
    assert_eq!(phones[2], "+48 345 345 345");
}
