use std::process::{ id };

use rusqlite::{ Result, types::{ FromSql, FromSqlError, FromSqlResult, ValueRef } };
use serde::{ Deserialize, Serialize };

use crate::{ db::database::Database, structs::HttpStatus };

#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    id: u32,
    name: String,
    author: String,
    img: Option<String>,
    song_list: SongList,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongId {
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongList(pub Vec<SongId>);

impl FromSql for SongList {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        // Получаем текст из колонки
        let text = value.as_str()?;

        let raw_list = serde_json
            ::from_str(text)
            .map_err(|err| FromSqlError::Other(Box::new(err)))?;

        Ok(SongList(raw_list))
    }
}

impl Database {
    pub fn create_playlist(
        name: String,
        author: String,
        img: Option<String>,
        song_list: SongList
    ) -> Result<String> {
        let data = Playlist {
            id: id(),
            name,
            author,
            img,
            song_list,
        };
        let cursor = Database::connect("database.db".to_string());
        let _ = cursor.execute(
            "INSERT INTO playlists (name, author, img, song_list) VALUES (?, ?, ?, ?)",
            [
                data.name,
                data.author,
                data.img.unwrap(),
                serde_json::to_string(&data.song_list).unwrap(),
            ]
        );
        let _ = cursor.close();
        Ok("Playlist created!".to_string())
    }

    pub fn update_playlist(value: String, condition: String) -> Result<String> {
        let cursor = Database::connect("database.db".to_string());
        let sql = format!("UPDATE playlists SET {} WHERE {}", value, condition);
        let _ = cursor.execute(sql.as_str(), []);
        let _ = cursor.close();
        Ok("Playlist updated!".to_string())
    }

    pub fn delete_playlist(id: u32) -> Result<String> {
        let cursor = Database::connect("database.db".to_string());
        let _ = cursor.execute("DELETE FROM playlists WHERE id = ?", [id]);
        let _ = cursor.close();
        Ok("Playlist deleted".to_string())
    }

    pub async fn get_all_playlists(condition: Option<String>) -> Result<Vec<Playlist>> {
        let res = tokio::task
            ::spawn_blocking(move || {
                let cursor = Database::connect("database.db".to_string());
                let sql;
                match condition {
                    Some(val) => {
                        sql = format!("SELECT * FROM playlists WHERE {}", val);
                    }
                    None => {
                        sql = String::from("SELECT * FROM playlists");
                    }
                }
                let mut statement = cursor
                    .prepare(sql.as_str())
                    .map_err(|e| { (HttpStatus::InternalError, e.to_string()) })
                    .expect("Failed to prepare the statement");
                let song_iter = statement
                    .query_map([], |row| {
                        Ok(Playlist {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            author: row.get(2)?,
                            img: row.get(3)?,
                            song_list: row.get(4)?,
                        })
                    })
                    .map_err(|e| (HttpStatus::InternalError, e.to_string()))
                    .unwrap();
                let mut songs: Vec<Playlist> = vec![];
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
}
