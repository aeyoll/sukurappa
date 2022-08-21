mod cache;
mod cli;
mod commands;
mod database;

use anyhow::Error;
use clap::Parser;
use cli::Cli;

use crate::cache::{create_cache_table, insert_cache, list_cache, remove_cache};
use crate::cli::Commands;
use crate::commands::add::AddCommand;
use crate::commands::list::ListCommand;
use crate::commands::remove::RemoveCommand;
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

fn run_app() -> Result<(), Error> {
    // Parse arguments
    let cli = Cli::parse();

    // Init database
    let connection = get_connection()?;
    create_cache_table(&connection)?;

    match &cli.command {
        Commands::List {} => ListCommand::run(&connection),
        Commands::Add { url, selector } => AddCommand::run(&connection, &url, &selector),
        Commands::Remove { url, selector } => RemoveCommand::run(&connection, &url, &selector),

        Commands::Watch {} => {
            todo!("Implement watch");
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
