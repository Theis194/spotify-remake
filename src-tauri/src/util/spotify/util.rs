use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::util::{
    config::Config,
    spotify_bb_error::BbError,
    spotify::response_objects::user::SpotifyUser,
};

pub async fn request<T>(url: String, token: String) -> Result<T, BbError> 
where 
    T: DeserializeOwned,
{
    let client = Client::new();

    let response = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    let response_text = response.text().await.unwrap();

    let deserialized_response: T = serde_json::from_str(&response_text)
        .map_err(|e| BbError::DeserializationError(e.to_string()))?;

    Ok(deserialized_response)
}

pub async fn request_user_profile(token: String) -> Result<SpotifyUser, BbError> {
    request::<SpotifyUser>("https://api.spotify.com/v1/me".to_string(), token).await
}