use rusqlite::{Connection, Result};

pub struct Database;

impl Database {
    pub fn initialize() -> Result<Connection> {
        println!("[INFO] Opening SQLite Database...");

        let connection = Connection::open("pingpongzzz.db")?;

        println!("[INFO] Database Connected Successfully");

        Ok(connection)
    }

    pub fn save_identity(
        connection: &Connection,
        hostname: &str,
        nickname: &str,
        fingerprint: &str,
    ) -> Result<()> {
        connection.execute(
            "
        INSERT INTO identity (hostname, nickname, fingerprint)
        VALUES (?1, ?2, ?3)
        ",
            (hostname, nickname, fingerprint),
        )?;

        Ok(())
    }


    pub fn close(connection: Connection) {
        drop(connection);
        println!("[INFO] Database Closed");
    }
}
