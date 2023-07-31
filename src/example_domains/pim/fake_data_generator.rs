#![allow(dead_code)]

use rand::Rng;

use super::model::{Address, Person};

static FIRSTNAMES: [&str; 16] = [
    "Artur",
    "Dawid",
    "Jakub",
    "Jan",
    "Kamil",
    "Krzysztof",
    "Łukasz",
    "Marcin",
    "Marek",
    "Mateusz",
    "Michał",
    "Paweł",
    "Piotr",
    "Stanisław",
    "Stefan",
    "Szymon",
];

static LASTNAMES: [&str; 5] = ["Kowalski", "Nowak", "Wiśniewski", "Malinowski", "Iksiński"];

static STREETS: [&str; 6] = [
    "Słoneczna",
    "Kościuszki",
    "Kręta",
    "Poziomkowa",
    "Marszałkowska",
    "Obrońców Helu",
];

fn get_random_person() -> Person {
    let mut rng = rand::thread_rng();
    Person {
        firstname: FIRSTNAMES[rng.gen_range(0..FIRSTNAMES.len())].to_string(),
        lastname: LASTNAMES[rng.gen_range(0..LASTNAMES.len())].to_string(),
        addresses: std::ops::Range {
            start: 0,
            end: rng.gen_range(1..2),
        }
        .map(|_| get_random_address())
        .collect(),
        phones: std::ops::Range {
            start: 0,
            end: rng.gen_range(1..2),
        }
        .map(|_| get_random_phone_number())
        .collect(),
    }
}

fn get_random_address() -> Address {
    let mut rng = rand::thread_rng();
    Address {
        street: STREETS[rng.gen_range(0..STREETS.len())].to_string(),
        home_number: rng.gen_range(1..100),
        flat_number: rng.gen_range(1..30),
    }
}

fn get_random_phone_number() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "+48 {}-{:0>3}-{:0>3}",
        rng.gen_range(100..999),
        rng.gen_range(0..999),
        rng.gen_range(0..999)
    )
}

pub fn generate_people(count: u32) -> Vec<Person> {
    log::info!("Generating {} people", count);
    let mut people = Vec::new();
    for _ in 0..count {
        people.push(get_random_person())
    }
    people
}

#[cfg(test)]
mod tests {
    use crate::example_domains::pim::fake_data_generator::{FIRSTNAMES, LASTNAMES, STREETS};

    use super::generate_people;

    #[test]
    fn can_generate_people() {
        let people = generate_people(2);
        assert_eq!(people.len(), 2);
        for person in people {
            assert!(FIRSTNAMES.contains(&person.firstname.as_str()));
            assert!(LASTNAMES.contains(&person.lastname.as_str()));
            assert!(!person.addresses.is_empty());
            for address in person.addresses {
                assert!(STREETS.contains(&address.street.as_str()));
                assert!(address.home_number > 0);
                assert!(address.flat_number > 0);
            }
            assert!(!person.phones.is_empty());
            for phone in person.phones {
                assert_eq!(phone.len(), 15);
            }
        }
    }
}
