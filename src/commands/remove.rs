use crate::remove_cache;
use rusqlite::Connection;

pub struct RemoveCommand;

impl RemoveCommand {
    pub fn run(connection: &Connection, url: &String, selector: &String) {
        match remove_cache(&connection, &url, &selector) {
            Ok(_) => println!("Url removed"),
            Err(_) => println!("Failed to remove url"),
        }
    }
}
