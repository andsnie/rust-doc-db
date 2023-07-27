# rust-doc-db

This is a prototype of a pseudo-document database implemented in Rust.

Features:
* documents stored in SQLite DB in field of JSON type
  + single document per row / field
  + supports querying by document fields via [SQLite JSON functions](https://www.sqlite.org/json1.html)
* data additionally stored in YAML files
  + allows versioning with Git
* same document can be reused across multiple domains
  + given document can be mapped to different domain types
  + documents fields unsupported / hidden in given domain are not overridden by other domain

## Code organization

* [src/doc_db](src/doc_db) - database engine
* [src/db](src/db) - default location for SQLite DB and YAML data files
* [src/example_domains](src/example_domains) - example use cases for the DB
* [src/config.rs](src/config.rs) - e.g. DB location

## Implementation notes

* currently documents are represented in code as [serde_json::Value](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html)

## Local development

* `cargo test` for running tests
* `cargo build` for building
* `cargo run -- --help` for checking available CLI commands (e.g. verifying or clearing existing DB)
* `cargo run -- verify-db` for creating new DB if it does not exist
* `cargo run -- generate-data` for filling existing DB with random data
* `cargo run -- clear-db` for removing all records from existing DB
