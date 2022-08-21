use crate::insert_cache;
use log::debug;

pub struct AddCommand;

impl AddCommand {
    /// Execute the "add" subcommand
    pub fn run(url: &str, selector: &str) {
        debug!("Running the \"add\" command");
        match insert_cache(url, selector, "") {
            Ok(_) => println!("Url added"),
            Err(_) => println!("Failed to add url"),
        }
    }
}
