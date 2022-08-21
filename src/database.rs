use rusqlite::Connection;

/// Initialize a database connection (stored in a file)
pub fn get_connection() -> Result<Connection, anyhow::Error> {
    let path = "./db.db3";
    let connection = Connection::open(path)?;
    Ok(connection)
}
