mod args;

use args::Args;
use clap::Parser;

fn parse(url: &str, selector: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let response = ureq::get(url).call()?.into_string()?;

    let document = scraper::Html::parse_document(&response);

    let element_selector = scraper::Selector::parse(selector).unwrap();

    let elements = document.select(&element_selector).map(|x| x.inner_html());

    let content = elements
        .into_iter()
        .next()
        .unwrap()
        .trim()
        .to_string();

    Ok(content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let url = args.url;
    let selector = args.selector;

    println!("Getting \"{}\" selector content on {}", selector, url);

    let content = parse(&url, &selector).unwrap();
    println!("{}", content);

    Ok(())
}