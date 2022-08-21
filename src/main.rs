mod cache;
mod cli;
mod commands;
mod database;

use anyhow::Error;
use clap::Parser;
use cli::Cli;
use log::debug;
use std::sync::Mutex;

extern crate log;
extern crate pretty_env_logger;

use crate::cache::{create_cache_table, insert_cache, list_cache, remove_cache};
use crate::cli::Commands;
use crate::commands::add::AddCommand;
use crate::commands::list::ListCommand;
use crate::commands::remove::RemoveCommand;
use crate::commands::watch::WatchCommand;
use crate::database::get_connection;

use once_cell::sync::Lazy;
use rusqlite::Connection;

static CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| Mutex::new(get_connection().unwrap()));

fn run_app() -> Result<(), Error> {
    // Parse arguments
    let cli = Cli::parse();

    // Init logger
    pretty_env_logger::init();
    debug!("Debug mode is on");

    // Init database
    create_cache_table()?;

    match &cli.command {
        Commands::List {} => ListCommand::run(),
        Commands::Add { url, selector } => AddCommand::run(url, selector),
        Commands::Remove { url, selector } => RemoveCommand::run(url, selector),
        Commands::Watch { frequency, command } => WatchCommand::run(frequency, command.to_string()),
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
