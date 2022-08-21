use crate::CONNECTION;
use cli_table::Table;

/// Stores the last fetched content for a combination of an url
/// and a selector
#[derive(Debug, Table)]
pub struct Cache {
    #[table(title = "Url")]
    pub url: String,

    #[table(title = "Selector")]
    pub selector: String,

    #[table(skip)]
    pub content: String,
}

/// Create the cache table in SQLite if not already existing
pub fn create_cache_table() -> Result<(), anyhow::Error> {
    let connection = CONNECTION.lock().unwrap();
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

/// Return a list of Cache instances
pub fn list_cache() -> Result<Vec<Cache>, anyhow::Error> {
    let connection = CONNECTION.lock().unwrap();
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

/// Try to find a Cache instance from an url + selector combination in
/// the SQLite database
pub fn search_cache(url: &str, selector: &str, content: &str) -> Result<Cache, anyhow::Error> {
    let connection = CONNECTION.lock().unwrap();
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
            insert_cache(url, selector, content)?;

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

/// Insert a Cache instance into the SQLite database
pub fn insert_cache(url: &str, selector: &str, content: &str) -> Result<(), anyhow::Error> {
    let connection = CONNECTION.lock().unwrap();
    connection.execute(
        "INSERT INTO cache (url, selector, content) VALUES (?1, ?2, ?3)",
        (&url, &selector, &content),
    )?;

    Ok(())
}

/// Update a Cache instance in the SQLite database
pub fn update_cache(url: &str, selector: &str, content: &str) -> Result<(), anyhow::Error> {
    let connection = CONNECTION.lock().unwrap();
    connection.execute(
        "UPDATE cache SET content = ?3 WHERE url = ?1 AND selector = ?2",
        (&url, &selector, &content),
    )?;

    Ok(())
}

/// Remove a Cache instance from the SQLite database
pub fn remove_cache(url: &str, selector: &str) -> Result<(), anyhow::Error> {
    let connection = CONNECTION.lock().unwrap();
    connection.execute(
        "DELETE FROM cache WHERE url = ?1 AND selector = ?2",
        (&url, &selector),
    )?;

    Ok(())
}
