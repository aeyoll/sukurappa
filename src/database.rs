use rusqlite::Connection;

pub fn get_connection() -> Result<Connection, anyhow::Error> {
    let path = "./db.db3";
    let connection = Connection::open(path)?;
    Ok(connection)

}