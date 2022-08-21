use cli_table::{print_stdout, WithTitle};
use rusqlite::Connection;
use crate::list_cache;

pub struct ListCommand;

impl ListCommand {
    pub fn run(connection: &Connection) {
        match list_cache(&connection) {
            Ok(caches) => print_stdout(caches.with_title()).unwrap(),
            Err(_) => {}
        }
    }
}