use cli_table::Table;
use rusqlite::Connection;

#[derive(Debug, Table)]
pub struct Cache {
    #[table(title = "Url")]
    pub(crate) url: String,

    #[table(title = "Selector")]
    pub(crate) selector: String,

    #[table(skip)]
    pub(crate) content: String,
}

impl PartialEq for Cache {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.selector == other.selector && self.content == other.content
    }
}

pub fn create_cache_table(connection: &Connection) -> Result<(), anyhow::Error> {
    // Create cache table
    connection.execute(
        "CREATE TABLE IF NOT EXISTS cache (
            url      TEXT NOT NULL,
            selector TEXT NOT NULL,
            content  TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;

    Ok(())
}

pub fn list_cache(connection: &Connection) -> Result<Vec<Cache>, anyhow::Error> {
    let mut stmt = connection.prepare("SELECT url, selector, content FROM cache")?;
    let caches: Vec<Cache> = stmt
        .query_map([], |row| {
            Ok(Cache {
                url: row.get(0)?,
                selector: row.get(1)?,
                content: row.get(2)?,
            })
        })
        .unwrap()
        .filter_map(|x| x.ok())
        .collect();

    Ok(caches)
}

pub fn search_cache(
    connection: &Connection,
    url: &str,
    selector: &str,
    content: &str,
) -> Result<Cache, anyhow::Error> {
    let mut stmt = connection.prepare(
        "SELECT url, selector, content FROM cache WHERE url = :url AND selector = :selector",
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
            insert_cache(&connection, &url, &selector, &content)?;

            Cache {
                url: url.to_string(),
                selector: selector.to_string(),
                content: content.to_string(),
            }
        }
        _ => panic!("Error while fetching cache"), // @TODO: improve me
    };

    Ok(cache)
}

pub fn insert_cache(
    connection: &Connection,
    url: &str,
    selector: &str,
    content: &str,
) -> Result<(), anyhow::Error> {
    connection.execute(
        "INSERT INTO cache (url, selector, content) VALUES (?1, ?2, ?3)",
        (&url, &selector, &content),
    )?;

    Ok(())
}

pub fn update_cache(
    connection: &Connection,
    url: &str,
    selector: &str,
    content: &str,
) -> Result<(), anyhow::Error> {
    connection.execute(
        "UPDATE cache SET content = ?3 WHERE url = ?1 AND selector = ?2",
        (&url, &selector, &content),
    )?;

    Ok(())
}

pub fn remove_cache(
    connection: &Connection,
    url: &str,
    selector: &str,
) -> Result<(), anyhow::Error> {
    connection.execute(
        "DELETE FROM cache WHERE url = ?1 AND selector = ?2",
        (&url, &selector),
    )?;

    Ok(())
}
