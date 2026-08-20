use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub path: String
}

impl Database {
    pub fn init(&self) -> Result<()> {
        let cursor = Database::connect("database.db".to_string());

        let _ = cursor.execute(
            "CREATE TABLE IF NOT EXISTS songs (
                id INTEGER PRIMARY KEY,
                title TEXT,
                artist TEXT,
                img TEXT,
                duration TEXT
            )",
            []
        );

        let _ = cursor.execute(
            "CREATE TABLE IF NOT EXISTS playlists (
                id INTEGER PRIMARY KEY,
                name TEXT,
                author TEXT,
                img TEXT,
                song_list TEXT
            )",
            []
        );

        let _ = cursor.close();

        Ok(())
    }

    pub fn connect(path: String) -> Connection {
        Connection::open(path.clone()).expect("Failed to connect to the database")
    }

    pub fn new(path: String) -> Self {
        Database { path }
    }
}