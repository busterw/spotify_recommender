use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::mood;
#[derive(Deserialize)]
struct Recommendations{
    tracks: Vec<Track>
}

#[derive(Deserialize, Serialize)]
pub struct Track{
    pub name:String,
    pub artists: Vec<Artist>,
    pub album: Album,
    pub external_urls: ExternalUrls
}

#[derive(Deserialize, Serialize)]
pub struct Artist{
    pub name: String
}

#[derive(Deserialize, Serialize)]
pub struct Album{
    pub images: Vec<Image>, // A list of images (Spotify provides multiple sizes)
}

#[derive(Deserialize, Serialize)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct ExternalUrls {
    pub spotify: String, // Spotify's URL for the track
}


pub async fn get_recommendations(token: &str, mood: &str, genre: &str) -> Result<Vec<Track>, Box<dyn std::error::Error>>{
    
    let mood_struct = match mood::mood_to_params(mood)? {
        Some(mood_struct) => mood_struct,
        None => {
            eprintln!("Mood '{}' not found in database.", mood);
            return Err("Mood not found".into());
        }
    };

    let client = Client::new();

    let response = client
    .get("https://api.spotify.com/v1/recommendations")
    .bearer_auth(token)
    .query(&[
        ("limit", "10"),
        ("seed_genres", genre),
        ("target_valence", &mood_struct.valence.to_string())
    ])
    .send()
    .await?;

    let recommendations: Recommendations = response.json().await?;

    Ok(recommendations.tracks)
}