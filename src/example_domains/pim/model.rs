use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub firstname: String,
    pub lastname: String,
    pub addresses: Vec<Address>,
    pub phones: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub home_number: i32,
    pub flat_number: i32,
}
