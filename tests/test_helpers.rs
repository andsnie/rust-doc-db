use std::sync::Once;

use rust_doc_db::doc_db::{clear_db, make_sure_db_exists, DbConfig};

static INIT: Once = Once::new();

pub fn get_test_config() -> DbConfig {
    
    DbConfig {
        sqlite_db_full_filename: "tmp/test_db/data.db".to_string(),
        text_db_path: "tmp/test_db/files/".to_string(),
    }
}

pub fn setup_tests_suite() {
    INIT.call_once(|| {
        simple_logger::SimpleLogger::new().env().init().unwrap();
    });
}

pub fn setup_test() {
    setup_tests_suite();
    let db_config = get_test_config();
    make_sure_db_exists(&db_config).unwrap();
    let _ = clear_db(&db_config);
}
