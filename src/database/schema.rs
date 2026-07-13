use rusqlite::{Connection, Result};

pub struct Tables;

impl Tables {
    pub fn create(connection: &Connection) -> Result<()> {

        connection.execute(
            "
            CREATE TABLE IF NOT EXISTS identity (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hostname TEXT NOT NULL,
                nickname TEXT NOT NULL,
                fingerprint TEXT NOT NULL
            )
            ",
            [],
        )?;

        println!("[INFO] Identity Table Created");

        Ok(())
    }
}