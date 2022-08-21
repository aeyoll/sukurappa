use crate::insert_cache;

pub struct AddCommand;

impl AddCommand {
    pub fn run(url: &str, selector: &str) {
        match insert_cache(url, selector, "") {
            Ok(_) => println!("Url added"),
            Err(_) => println!("Failed to add url"),
        }
    }
}
