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
    /// Do an action when the content of a webpage changes
    Watch {
        /// The duration is seconds to check for website changes
        #[clap(short, long, default_value_t = 60)]
        frequency: u32,

        /// The command to run when the content changes. The string
        /// "NEW_CONTENT" will be updated with the actual new content
        #[clap(short, long)]
        command: String,
    },

    /// List all websites in the watch list
    List {},

    /// Add a website to the watch list
    Add {
        /// The url of the website (with scheme)
        #[clap(short, long)]
        url: String,

        /// The CSS selector to observe. If the selector is present multiple
        /// times, the first one will be used.
        #[clap(short, long)]
        selector: String,
    },
    /// Remove a website from the watch list
    Remove {
        /// The url of the website (with scheme)
        #[clap(short, long)]
        url: String,

        /// The CSS selector to remove
        #[clap(short, long)]
        selector: String,
    },
}
