use crate::list_cache;
use cli_table::{print_stdout, WithTitle};
use log::{debug, error, info};
use rusqlite::Connection;

pub struct ListCommand;

impl ListCommand {
    pub fn run(connection: &Connection) {
        debug!("Running the \"list\" command");
        match list_cache(connection) {
            Ok(caches) => {
                if caches.len() > 0 {
                    print_stdout(caches.with_title()).unwrap();
                } else {
                    info!("Nothing to display");
                }
            }
            Err(_) => error!("Failed to fetch the list of websites"),
        }
    }
}
