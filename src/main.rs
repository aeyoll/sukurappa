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
use crate::commands::watch::WatchCommand;
use crate::database::get_connection;

fn run_app() -> Result<(), Error> {
    // Parse arguments
    let cli = Cli::parse();

    // Init database
    let connection = get_connection()?;
    create_cache_table(&connection)?;

    match &cli.command {
        Commands::List {} => ListCommand::run(&connection),
        Commands::Add { url, selector } => AddCommand::run(&connection, url, selector),
        Commands::Remove { url, selector } => RemoveCommand::run(&connection, url, selector),
        Commands::Watch { frequency } => WatchCommand::run(frequency),
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
