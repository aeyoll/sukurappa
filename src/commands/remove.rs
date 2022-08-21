use crate::remove_cache;
use rusqlite::Connection;

pub struct RemoveCommand;

impl RemoveCommand {
    pub fn run(connection: &Connection, url: &str, selector: &str) {
        match remove_cache(connection, url, selector) {
            Ok(_) => println!("Url removed"),
            Err(_) => eprintln!("Failed to remove url"),
        }
    }
}
