use crate::list_cache;
use cli_table::{print_stdout, WithTitle};
use log::{debug, error, info};

pub struct ListCommand;

impl ListCommand {
    pub fn run() {
        debug!("Running the \"list\" command");
        match list_cache() {
            Ok(caches) => {
                if !caches.is_empty() {
                    print_stdout(caches.with_title()).unwrap();
                } else {
                    info!("Nothing to display");
                }
            }
            Err(_) => error!("Failed to fetch the list of websites"),
        }
    }
}
