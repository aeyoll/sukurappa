mod args;
mod cache;
mod database;

use anyhow::anyhow;
use args::Args;
use clap::Parser;

use crate::cache::{create_cache_table, search_cache, update_cache};
use crate::database::get_connection;

fn parse(url: &str, selector: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Fetch the documents html
    // @TODO: manage http error codes
    let response = ureq::get(url).call()?.into_string()?;

    let document = scraper::Html::parse_document(&response);
    let element_selector = scraper::Selector::parse(selector).unwrap();
    let elements = document.select(&element_selector).map(|x| x.inner_html());

    let content = elements.into_iter().next().unwrap().trim().to_string();

    Ok(content)
}

fn run_app() -> Result<(), anyhow::Error> {
    // Parse arguments
    let args = Args::parse();
    let url: String = args.url;
    let selector: String = args.selector;

    // Fetching content from webpage
    let content = parse(&url, &selector).unwrap();

    // Init database
    let connection = get_connection()?;
    create_cache_table(&connection)?;

    // Search for cache
    let cache = search_cache(&connection, &url, &selector, &content)?;

    if cache.content == content {
        // Unchanged content
        Err(anyhow!("Content is the same, doing nothing"))
    } else {
        println!("Content is different, doing something");
        update_cache(&connection, &url, &selector, &content)?;
        Ok(())
    }
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            println!("{}", err.to_string());
            1
        }
    });
}
