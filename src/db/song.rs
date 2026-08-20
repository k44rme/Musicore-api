extern crate rusqlite;

use rusqlite::{ Result };
use serde::{ Deserialize, Serialize };

use crate::{ HttpStatus, db::{ database::{ Database } } };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    pub id: u32,
    pub title: String,
    pub artist: String,
    pub img: Option<String>,
    pub duration: String,
}

impl Database {
    pub fn insert_song(data: Song) -> Result<String> {
        let cursor = Database::connect("database.db".to_string());
        let _ = cursor.execute(
            "INSERT INTO songs (title, artist, img, duration) VALUES (?, ?, ?, ?)",
            (data.title, data.artist, data.img, data.duration)
        );
        let _ = cursor.close();
        Ok("Song inserted!".to_string())
    }

    pub fn update_song(params: (&str, &str)) -> Result<String> {
        let cursor = Database::connect("database.db".to_string());
        let sql = format!("UPDATE songs SET {} WHERE {}", params.0, params.1);
        let _ = cursor.execute(sql.as_str(), []);
        let _ = cursor.close();
        Ok("Song updated!".to_string())
    }

    pub async fn get_all_songs(condition: Option<String>) -> Result<Vec<Song>> {
        let res = tokio::task
            ::spawn_blocking(move || {
                let cursor = Database::connect("database.db".to_string());
                let sql;
                match condition {
                    Some(val) => {
                        sql = format!("SELECT * FROM songs WHERE {}", val);
                    }
                    None => {
                        sql = String::from("SELECT * FROM songs");
                    }
                }
                let mut statement = cursor
                    .prepare(sql.as_str())
                    .map_err(|e| { (HttpStatus::InternalError, e.to_string()) })
                    .expect("Failed to prepare the statement");
                let song_iter = statement
                    .query_map([], |row| {
                        Ok(Song {
                            id: row.get(0)?,
                            title: row.get(1)?,
                            artist: row.get(2)?,
                            img: row.get(3)?,
                            duration: row.get(4)?,
                        })
                    })
                    .map_err(|e| (HttpStatus::InternalError, e.to_string()))
                    .unwrap();
                let mut songs: Vec<Song> = vec![];
                for song in song_iter {
                    songs.push(
                        song.map_err(|e| (HttpStatus::InternalError, e.to_string())).unwrap()
                    );
                }
                songs
            }).await
            .unwrap();

        Ok(res)
    }

    pub fn delete_song(id: u32) -> Result<String> {
        let cursor = Database::connect("database.db".to_string());
        let sql = format!("DELETE FROM songs WHERE id = {}", id);
        let _ = cursor.execute(sql.as_str(), []);
        Ok("Song Deleted!".to_string())
    }
}
