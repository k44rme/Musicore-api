use serde::{ Deserialize, Serialize };

use crate::db::playlist::{ SongList };




// ----------- HTTP -----------------




#[derive(Serialize, Deserialize, Debug)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub response_message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalError = 500,
}



// --------- SONGS -------------



#[derive(Serialize, Deserialize, Debug)]
pub struct RequestedSong {
    pub title: String,
    pub artist: String,
    pub img: Option<String>,
    pub duration: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDeleteSong {
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestUpdateSong {
    pub statement: String,
    pub condition: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RequestedAllSongs {
    pub condition: String,
}



// ---------- PLAYLISTS ------------




#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedCreatePLaylist {
    pub name: String,
    pub author: String,
    pub img: Option<String>,
    pub song_list: SongList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedUpdatePlaylist {
    pub statement: String,
    pub condition: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RequestedAllPlaylists {
    pub condition: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDeletePlaylist {
    pub id: u32,
}
