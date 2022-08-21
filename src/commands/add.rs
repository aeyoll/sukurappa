use crate::insert_cache;
use rusqlite::Connection;

pub struct AddCommand;

impl AddCommand {
    pub fn run(connection: &Connection, url: &str, selector: &str) {
        match insert_cache(connection, url, selector, "") {
            Ok(_) => println!("Url added"),
            Err(_) => println!("Failed to add url"),
        }
    }
}
