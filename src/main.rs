mod cache;
mod cli;
mod database;

use anyhow::{anyhow, Error};
use clap::Parser;
use cli::Cli;
use cli_table::{print_stdout, WithTitle};

use crate::cache::{Cache, create_cache_table, insert_cache, list_cache, remove_cache, search_cache, update_cache};
use crate::cli::Commands;
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
    let cli = Cli::parse();

    // Init database
    let connection = get_connection()?;
    create_cache_table(&connection)?;

    match &cli.command {
        Commands::Add { url, selector } => match insert_cache(&connection, &url, &selector, "") {
            Ok(_) => println!("Url added"),
            Err(_) => println!("Failed to add url"),
        },

        Commands::Remove { url, selector } => match remove_cache(&connection, &url, &selector) {
            Ok(_) => println!("Url removed"),
            Err(_) => println!("Failed to remove url"),
        },

        Commands::Watch {} => {
            todo!("Implement watch");
        }

        Commands::List {} => {
            match list_cache(&connection) {
                Ok(caches) => print_stdout(caches.with_title())?,
                Err(_) => {}
            }
        }
    }

    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(_err) => {
            // println!("{}", _err.to_string());
            1
        }
    });
}
