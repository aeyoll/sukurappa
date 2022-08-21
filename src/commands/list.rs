use crate::list_cache;
use cli_table::{print_stdout, WithTitle};
use rusqlite::Connection;

pub struct ListCommand;

impl ListCommand {
    pub fn run(connection: &Connection) {
        match list_cache(connection) {
            Ok(caches) => print_stdout(caches.with_title()).unwrap(),
            Err(_) => eprintln!("Nothing to display"),
        }
    }
}
