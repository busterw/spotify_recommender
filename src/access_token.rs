use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct TokenResponse{
    access_token : String,
    token_type : String,
    expires_in : u64
}

pub async fn get_access_token(client_id: &str, client_secret : &str) -> Result<String, Box<dyn std::error::Error>> 
{
    let client = Client::new();
    let params = [
        ("grant_type", "client_credentials")
        ];

        let response = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
        .await?;

        let token_response: TokenResponse = response.json().await?;

        Ok(token_response.access_token)
}