#![allow(dead_code)]

use std::collections::HashMap;

use crate::doc_db::{get_entries_from_db, DbConfig, DocDbResult};

use self::model::Person;

pub mod fake_data_generator;
pub mod model;

pub fn get_phones_of_people_by_firstname(
    firstname: &str,
    db_config: &DbConfig,
) -> DocDbResult<Vec<String>> {
    let mut where_clause_params: HashMap<&str, &str> = HashMap::new();
    where_clause_params.insert("firstname", firstname);
    let entities = get_entries_from_db(
        "json_extract(content, '$.firstname') = :firstname",
        where_clause_params,
        db_config,
    )?;

    let mut phones: Vec<String> = Vec::new();
    for entry in entities {
        let person: Person = serde_json::from_value(entry.entity)?;
        for phone in person.phones {
            if !phones.contains(&phone) {
                phones.push(phone);
            }
        }
    }
    Ok(phones)
}
