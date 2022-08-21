use crate::remove_cache;

pub struct RemoveCommand;

impl RemoveCommand {
    pub fn run(url: &str, selector: &str) {
        match remove_cache(url, selector) {
            Ok(_) => println!("Url removed"),
            Err(_) => eprintln!("Failed to remove url"),
        }
    }
}
