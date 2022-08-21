use anyhow::Error;
use std::thread;
use std::time::Duration;

use crate::cache::{search_cache, update_cache, Cache};
use crate::list_cache;
use clokwerk::{Scheduler, TimeUnits};
use duct::cmd;

fn parse(url: &str, selector: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Fetch the documents html
    let response = ureq::get(url).call()?.into_string()?;

    let document = scraper::Html::parse_document(&response);
    let element_selector = scraper::Selector::parse(selector).unwrap();
    let elements = document.select(&element_selector).map(|x| x.inner_html());

    let content = elements.into_iter().next().unwrap().trim().to_string();

    Ok(content)
}

fn process(command: &str) -> Result<(), Error> {
    let caches: Vec<Cache> = list_cache().unwrap();

    caches.into_iter().for_each(|cache| {
        // Fetching content from webpage
        let content = match parse(&cache.url, &cache.selector) {
            Ok(content) => content,
            Err(_e) => {
                eprintln!("Failed to get content for {}", &cache.url);
                return;
            }
        };

        // Search for cache
        let cache = search_cache(&cache.url, &cache.selector, &content).unwrap();

        if cache.content != content {
            println!("Content updated for url: {}", &cache.url);
            update_cache(&cache.url, &cache.selector, &content).unwrap();

            let new_command = command
                .replace("NEW_CONTENT", format!("\"{}\"", content).as_str())
                .replace("URL", &cache.url);
            match cmd!("bash", "-c", &new_command).run() {
                Ok(_) => println!("Successfully ran {}", &new_command),
                Err(_) => eprintln!("Failed to run {}", &new_command),
            }
        }
    });

    Ok(())
}

pub struct WatchCommand;

impl WatchCommand {
    pub fn run(frequency: &u32, command: String) {
        // Create a new scheduler
        let mut scheduler = Scheduler::new();
        scheduler
            .every(frequency.second())
            .run(move || process(&command).unwrap());

        loop {
            scheduler.run_pending();
            thread::sleep(Duration::from_millis(100));
        }
    }
}
