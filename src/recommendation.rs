use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Track{
    pub name:String,
    pub artists: Vec<Artist>
}

#[derive(Deserialize)]
pub struct Artist{
    pub name: String
}

#[derive(Deserialize)]
struct Recommendations{
    tracks: Vec<Track>
}

struct MoodParams{
    energy: f32,
    liveness: f32,
    dance: f32
}

fn mood_to_params(mood: &str) -> MoodParams {
    match mood.to_lowercase().as_str() {
        "happy" => MoodParams {
            dance: 0.8,
            energy: 0.6,
            liveness: 0.08
        },
        "sad" => MoodParams {
            dance: 0.2,
            energy: 0.3,
            liveness: 0.08
        },
        _ => MoodParams{
            dance: 0.5,
            energy: 0.5,
            liveness: 0.08
        }
    }
}

pub async fn get_recommendations(token: &str, mood: &str) -> Result<Vec<Track>, Box<dyn std::error::Error>>{
    
    let mood_struct = mood_to_params(mood);

    let client = Client::new();

    let response = client
    .get("https://api.spotify.com/v1/recommendations")
    .bearer_auth(token)
    .query(&[
        ("limit", "10"),
        ("seed_genres", "pop"),
        ("target_energy", &mood_struct.energy.to_string()),
        ("target_liveness", &mood_struct.liveness.to_string()),
        ("target_danceability", &mood_struct.dance.to_string())
    ])
    .send()
    .await?;

    let recommendations: Recommendations = response.json().await?;

    Ok(recommendations.tracks)
}