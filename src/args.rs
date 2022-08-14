use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// url
    #[clap(short, long)]
    pub url: String,

    /// selector
    #[clap(short, long)]
    pub selector: String,
}
