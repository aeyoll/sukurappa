mod args;
mod cache;

use args::Args;
use cache::Cache;
use clap::Parser;

use crate::cache::{create_cache_table, insert_cache, search_cache};
use rusqlite::{Connection, Result};

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

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let url: String = args.url;
    let selector: String = args.selector;

    println!("Getting \"{}\" selector content on {}", selector, url);

    let content = parse(&url, &selector).unwrap();

    let path = "./db.db3";
    let connection = Connection::open(path)?;

    // Create cache table
    create_cache_table(&connection)?;
    let cache = search_cache(&connection, &url, &selector, &content)?;

    if cache.content == content {
        // Unchanged content
        println!("Content is the same, doing nothing");
    } else {
        println!("Content is different, doing something");
    }

    Ok(())
}
