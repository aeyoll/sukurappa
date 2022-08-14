mod args;

use args::Args;
use clap::Parser;

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Cache {
    url: String,
    selector: String,
    content: String,
}

impl PartialEq for Cache {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.selector == other.selector && self.content == other.content
    }
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let url = args.url;
    let selector = args.selector;

    println!("Getting \"{}\" selector content on {}", selector, url);

    let content = parse(&url, &selector).unwrap();

    let path = "./db.db3";
    let connection = Connection::open(path)?;

    // Create cache table
    connection.execute(
        "CREATE TABLE IF NOT EXISTS cache (
            url      TEXT NOT NULL,
            selector TEXT NOT NULL,
            content  TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;

    let mut stmt = connection.prepare(
        "SELECT url, selector, content FROM cache where url = :url AND selector = :selector",
    )?;
    let cache: Cache = match stmt.query_row(&[(":url", &url), (":selector", &selector)], |row| {
        Ok(Cache {
            url: row.get(0)?,
            selector: row.get(1)?,
            content: row.get(2)?,
        })
    }) {
        Ok(cache) => cache,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            connection.execute(
                "INSERT INTO cache (url, selector, content) VALUES (?1, ?2, ?3)",
                (&url, &selector, &content),
            )?;

            Cache {
                url,
                selector,
                content: content.to_string(),
            }
        }
        _ => panic!("Error while fetching cache"), // @TODO: improve me
    };

    if cache.content == content {
        // Unchanged content
        println!("Content is the same, doing nothing");
    } else {
        println!("Content is different, doing something");
    }

    Ok(())
}
