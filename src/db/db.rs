// use rocket::log::private::info;
use log::{error, info};
use rusqlite::Connection;
use std::{env, fs, path::Path};

pub fn connect() -> Connection {
    let mut must_initialize_db = false;
    if !Path::new("dd.db").exists() {
        must_initialize_db = true;
    }

    let connection = Connection::open("dd.db").unwrap_or_else(|e| {
        error!(target: "error", "failed to open database: {}", e);
        panic!("database access error")
    });

    if must_initialize_db {
        let line_ending = match env::consts::OS {
            "windows" => ";\r\n",
            _ => ";\n",
        };
        let query = fs::read_to_string("init.sql").expect("initial schema does not exist");
        let commands = query.split(line_ending);

        for command in commands {
            connection.execute(command, ()).unwrap_or_else(|e| {
                error!("failed to execute command '{}': {}", command, e);
                panic!("command execution failure")
            });
        }

        info!(target: "info", "database created");
    }
    connection
}
