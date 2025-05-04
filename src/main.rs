use std::{fs, path::Path};

use databases::connection::Database;
use helpers::spinner;

mod databases;
mod helpers;

enum DatabaseInstanceType {
    Mongo,
    Postgres,
}
fn main() {
    let target_database_name = "primary-v4";
    let source_database_name = "primary";
    let source_connection_string = "mongodb://ws-staging:pBUQtArBlinM7klihghD3@deployment-staging-mongo1:20204,deployment-staging-mongo2:20205,deployment-staging-mongo3:20206/?authSource=admin";
    let target_connection_string = "mongodb://ws-dev:pBUQtArBvtnM7akshghD3@deployment-mongo1:20201,deployment-mongo2:20202,deployment-mongo3:20203/?authSource=admin";

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
