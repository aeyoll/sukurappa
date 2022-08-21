use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Watch {},
    List {},
    Add {
        #[clap(short, long)]
        url: String,

        /// selector
        #[clap(short, long)]
        selector: String,
    },
    Remove {
        #[clap(short, long)]
        url: String,

        /// selector
        #[clap(short, long)]
        selector: String,
    },
}
