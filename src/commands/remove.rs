use crate::remove_cache;
use log::{debug, error, info};

pub struct RemoveCommand;

impl RemoveCommand {
    /// Execute the "remove" subcommand
    pub fn run(url: &str, selector: &str) {
        debug!("Running the \"remove\" command");

        match remove_cache(url, selector) {
            Ok(_) => info!("Url removed"),
            Err(_) => error!("Failed to remove url"),
        }
    }
}
