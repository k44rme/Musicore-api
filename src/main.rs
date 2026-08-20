use axum::{ Json, Router, extract::Query, routing::{ delete, get, patch, post } };
use std::{ process::id };
use tokio;

mod db;
mod structs;

use db::song::{ Song };
use crate::{
    db::{ database::Database, playlist::{ Playlist, SongId, SongList } }, structs::{
        HttpResponse,
        HttpStatus,
        QueryDeletePlaylist,
        QueryDeleteSong,
        RequestUpdateSong,
        RequestedAllPlaylists,
        RequestedAllSongs,
        RequestedCreatePLaylist,
        RequestedSong,
        RequestedUpdatePlaylist,
    },
};

#[tokio::main]
async fn main() {
    let _ = Database::new("database.db".to_string()).init();
    let app = Router::new()
        .route(
            "/",
            get(|| async { "Root page" })
        )
        // Song routes
        .route("/song/all", get(songs))
        .route("/song/new", post(add_song))
        .route("/song/update", patch(update_song))
        .route("/song/delete", delete(delete_song))
        // Playlist routes
        .route("/playlist/all", get(playlists))
        .route("/playlist/new", post(create_playlist))
        .route("/playlist/update", patch(update_playlist))
        .route("/playlist/delete", delete(delete_playlist));
    let addr = "0.0.0.0:618";
    println!("\n     Server started in address: {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ------- SONGS HANDLERS --------------

async fn songs(Json(body): Json<RequestedAllSongs>) -> Json<Vec<Song>> {
    /* let res = Song {
        id: id(),
        title: "My Name Is".to_string(),
        artist: "Eminem".to_string(),
        img: None,
        duration: "0:30".to_string(),
    }; */
    let mut condition: Option<String> = Some(String::from(&body.condition));
    if body.condition == "" {
        condition = None;
    }
    let res = Database::get_all_songs(condition).await.unwrap();
    axum::Json::from(res)
}

async fn add_song(Json(body): Json<RequestedSong>) -> Json<HttpResponse> {
    /* let client = YTMusicClient::builder().build().unwrap();
    let song: ytmusicapi::Song = client.get_song("dQw4w9WgXcQ").await.unwrap(); */
    let data = Song {
        id: id(),
        title: body.title,
        artist: body.artist,
        img: body.img,
        duration: body.duration,
    };
    let res = Database::insert_song(data).expect("Failed to insert song");
    let result = HttpResponse {
        status: HttpStatus::Ok,
        response_message: res,
    };
    axum::Json::from(result)
}

async fn update_song(Json(body): Json<RequestUpdateSong>) -> Json<HttpResponse> {
    let res = Database::update_song((body.statement.as_str(), body.condition.as_str())).expect(
        "Failed to update the song"
    );
    let result = HttpResponse {
        status: HttpStatus::Ok,
        response_message: res,
    };
    axum::Json::from(result)
}

async fn delete_song(Query(param): Query<QueryDeleteSong>) -> Json<HttpResponse> {
    let res = Database::delete_song(param.id).unwrap();
    let result = HttpResponse {
        status: HttpStatus::Ok,
        response_message: res,
    };
    axum::Json::from(result)
}

// ---------- PLAYLISTS HANDLERS -------------

async fn create_playlist(Json(body): Json<RequestedCreatePLaylist>) -> Json<HttpResponse> {
    let res = Database::create_playlist(
        body.name,
        body.author,
        body.img,
        SongList(vec![SongId { id: 2 }, SongId { id: 4 }])
    ).unwrap();
    let result = HttpResponse {
        status: HttpStatus::Ok,
        response_message: res,
    };
    axum::Json::from(result)
}

async fn update_playlist(Json(body): Json<RequestedUpdatePlaylist>) -> Json<HttpResponse> {
    let res = Database::update_playlist(body.statement, body.condition).unwrap();
    let result = HttpResponse {
        status: HttpStatus::Ok,
        response_message: res,
    };
    axum::Json::from(result)
}

async fn playlists(Json(body): Json<RequestedAllPlaylists>) -> Json<Vec<Playlist>> {
    let mut cond = Some(String::from(&body.condition));
    if body.condition == "" {
        cond = None;
    }
    let res = Database::get_all_playlists(cond).await.unwrap();
    axum::Json::from(res)
}

async fn delete_playlist(Query(param): Query<QueryDeletePlaylist>) -> Json<HttpResponse> {
    let res = Database::delete_playlist(param.id).unwrap();
    let result = HttpResponse {
        status: HttpStatus::Ok,
        response_message: res,
    };
    axum::Json::from(result)
}
