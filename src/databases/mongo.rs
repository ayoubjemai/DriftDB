use super::connection::Database;
use crate::helpers::spinner;
use std::process::Command;

pub struct Mongo<'a> {
    pub source_connection_string: &'a str,
    pub target_connection_string: &'a str,
    pub target_database_name: &'a str,
    pub source_database_name: &'a str,
}

impl<'a> Database for Mongo<'a> {
    fn validate(&self) -> bool {
        if !self.source_connection_string.contains("mongodb://") {
            panic!("sorce Connection String is NOTTTT VALID");
        }

        if !self.target_connection_string.contains("mongodb://") {
            panic!("target Connection String is NOTTTT VALID");
        }

        let mongodump_command = Command::new("mongodump").arg("--version").output();
        if !mongodump_command.is_ok() {
            panic!("Failed to load mongodump");
        }

        let mongorestore_command = Command::new("mongorestore").arg("--version").output();

        if !mongorestore_command.is_ok() {
            panic!("Failed to load mongorestore");
        }

        return true;
    }

    fn dump(&self, execluded_entity: Vec<&str>, dump_path: &str) {
        let execlude_entity_str_without_space = execluded_entity
            .iter()
            .map(|entity| format!("--excludeCollection={}", entity))
            .collect::<Vec<_>>()
            .join(" ");

        let mut mongodump_command = Command::new("mongodump");
        let mut command = mongodump_command
            .arg(format!("--uri={}", self.source_connection_string))
            .arg(format!("--out={}", dump_path))
            .arg(format!("--db={}", self.source_database_name))
            .args(execlude_entity_str_without_space.split_whitespace());

        let spinner = spinner::Spinner {};

        let command_output = spinner.execute_with_spinner(
            &mut command,
            &format!("Dumping database {}...", self.source_database_name),
        );

        if !command_output.status.success() {
            panic!("Failed to execute command: {:?}", command_output.stderr);
        }

        println!("Dump completed successfully.");
    }

    fn restor(&self, dum_path: &str) {
        let mut mongorestore_command = Command::new("mongorestore");
        let mut command = mongorestore_command
            .arg(format!("--uri={}", self.target_connection_string))
            .arg(format!(
                "--nsFrom={}",
                format!("{}.*", self.source_database_name)
            ))
            .arg(format!(
                "--nsTo={}",
                format!("{}.*", self.target_database_name)
            ))
            .arg(format!("--dir={}/{}", dum_path, self.source_database_name))
            .arg(format!("--db={}", self.target_database_name));
        let spinner = spinner::Spinner {};

        let command_output = spinner.execute_with_spinner(
            &mut command,
            &format!("Restoring database to {}...", self.source_database_name),
        );

        if !command_output.status.success() {
            panic!("Failed to execute command: {:?}", command_output.stderr);
        }

        println!("Restore completed successfully.");
    }
}
