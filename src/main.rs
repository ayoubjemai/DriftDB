use databases::connection::Database;
use dotenv::dotenv;
use std::{env, fs};

mod databases;
mod helpers;

enum DatabaseInstanceType {
    Mongo,
    Postgres,
}
fn main() {
    dotenv().ok();
    let env_target_database_name = getEnvVar("target_database_name");
    let target_database_name = env_target_database_name.as_str();
    let env_source_database_name = getEnvVar("source_database_name");
    let source_database_name = env_source_database_name.as_str();
    let env_source_connection_string = getEnvVar("source_connection_string");
    let source_connection_string = env_source_connection_string.as_str();
    let env_target_connection_string = getEnvVar("target_connection_string");
    let target_connection_string = env_target_connection_string.as_str();

    let database_instance: DatabaseInstanceType = DatabaseInstanceType::Mongo;

    let database = initialize_database_connection(
        target_database_name,
        source_database_name,
        source_connection_string,
        target_connection_string,
        &database_instance,
    );

    let dump_path = "tmp/mongo-dump";

    fs::create_dir_all(dump_path).expect("Failed to create dump directory");
    let execluded_entities = vec!["notifications", "notificationsettings"];

    database.validate();
    database.dump(execluded_entities, dump_path);
    database.restor(dump_path);

    fs::remove_dir_all(dump_path).expect("Failed to remove dump directory");
    println!("Dump and restore completed successfully.");
}

fn initialize_database_connection<'a>(
    target_database_name: &'a str,
    source_database_name: &'a str,
    source_connection_string: &'a str,
    target_connection_string: &'a str,
    databases_instnace: &DatabaseInstanceType,
) -> Box<dyn Database + 'a> {
    match databases_instnace {
        DatabaseInstanceType::Mongo => Box::new(databases::mongo::Mongo {
            source_connection_string,
            target_connection_string,
            target_database_name,
            source_database_name,
        }),
        _ => {
            panic!("Database instance not implemented yet");
        }
    }
}

fn getEnvVar(key: &str) -> String {
    match env::var(key) {
        Ok(value) => {
            if value.is_empty() {
                panic!("Environment variable {} is empty", key);
            }
            value
        }
        Err(_) => panic!("Environment variable {} not set", key),
    }
}
