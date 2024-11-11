use std::{io::{self, Write}, time::Instant};

use access_token::get_access_token;
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use recommendation::get_recommendations;
use serde::Deserialize;

mod access_token;
mod recommendation;
mod mood;

#[derive(Deserialize)]
struct QueryParams{
    mood: String,
    genre: String
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new( || {
        App::new()
            .route("/recommendations", web::get().to(get_recommendations_handler))
            .service(Files::new("/","./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


async fn get_recommendations_handler(params: web::Query<QueryParams>) -> impl Responder{
    let client_id = "cd8bff96f8544158868aa53aa4a7bd72";
    let client_secret = "d879110eade840b58a0da18a1dadb453";

    println!("I'm hit!");

    let token = get_access_token(client_id, client_secret).await.unwrap();

    let mood = &params.mood;
    let genre = &params.genre;

    let tracks = get_recommendations(&token, mood, genre).await.unwrap_or_default();

    HttpResponse::Ok().json(tracks)
}

