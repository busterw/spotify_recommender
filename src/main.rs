use std::{io::{self, Read, Write}, time::Instant};

use access_token::get_access_token;
use recommendation::get_recommendations;

mod access_token;
mod recommendation;

//TODO: should probably use 'valance' and 'energy' to measure mood, rather than 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client_id = "cd8bff96f8544158868aa53aa4a7bd72";
    let client_secret = "d879110eade840b58a0da18a1dadb453";

    let token = get_access_token(client_id, client_secret).await?;

    let mood = &get_user_mood();

    let start_time = Instant::now();

    let tracks = get_recommendations(&token, mood).await?;

    let elapsed = start_time.elapsed().as_secs_f64();

    println!();

    for track in tracks {
        println!("{} by {}", track.name, track.artists.first().unwrap().name);
    }

    println!("\n Time Taken to get tracks: {:?}s", elapsed);

    Ok(())

}

fn get_user_mood() -> String{
    print!("Enter your mood:");
    io::stdout().flush().unwrap();

    let mut mood = String::new();

    io::stdin().read_line(&mut mood).unwrap();

    mood.trim().to_lowercase()
}
