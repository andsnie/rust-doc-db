use clap::Parser;
use cli::{Cli, Commands};
use color_eyre::eyre::Result;
use doc_db::{clear_db, insert_entity_to_db, make_sure_db_exists, DbConfig};
use example_domains::pim::fake_data_generator::generate_people;
use serde_json::json;

mod cli;
mod config;
mod doc_db;
mod example_domains;

fn get_prod_db_config() -> DbConfig {
    DbConfig {
        sqlite_db_full_filename: config::SQLITE_DB_FULL_FILENAME.to_string(),
        text_db_path: config::YAML_FILES_ROOT_PATH.to_string(),
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    simple_logger::SimpleLogger::new().env().init()?;

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::VerifyDb {}) => {
            let db_config = get_prod_db_config();
            match make_sure_db_exists(&db_config) {
                Ok(_) => {
                    log::info!("DB verified correctly");
                }
                Err(e) => {
                    log::error!("Unable to verify / reinit DB {}", e);
                }
            }
        }
        Some(Commands::ClearDb {}) => {
            let db_config = get_prod_db_config();
            match make_sure_db_exists(&db_config) {
                Ok(_) => match clear_db(&db_config) {
                    Ok(_) => {
                        log::info!("DB cleared");
                    }
                    Err(e) => {
                        log::error!("Unable to clear DB: {}", e);
                    }
                },
                Err(e) => {
                    log::error!("Unable to verify / reinit DB {}", e);
                }
            }
        }
        Some(Commands::GenerateData {}) => {
            let db_config = get_prod_db_config();
            const PEOPLE_COUNT: u32 = 100;
            let people = generate_people(PEOPLE_COUNT);
            for person in people {
                let json_person = json!(person);
                match insert_entity_to_db(&json_person, &db_config) {
                    Ok(_) => {}
                    Err(e) => log::error!("Unable to save person: {}", e),
                }
            }
        }
        None => {}
    }
    Ok(())
}
