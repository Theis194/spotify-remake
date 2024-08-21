use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::de::DeserializeOwned;
use shared_lib::shared::spotify_objects::user::SpotifyUser;

use crate::util::spotify_bb_error::BbError;

/// Generic function to make a request to the Spotify API
/// 
/// * `url` - The URL to make the request to
/// * `token` - The user's Spotify access token
/// * `client` - The reqwest client
/// 
/// # Returns
/// 
/// The deserialized response from the Spotify API
/// 
/// # Errors
/// 
/// If the request fails or the response cannot be deserialized
/// 
/// # Example
/// 
/// ```
/// use reqwest::Client;
/// use serde::de::DeserializeOwned;
/// 
/// use crate::util::spotify::util::request;
/// 
/// #[tokio::main]
/// async fn main() {
///    let client = Client::new();
///    let token= "BQD1".to_string();
///    let response: SpotifyUser = request::<SpotifyUser>("https://api.spotify.com/v1/me".to_string(), token, &client).await.unwrap();
/// 
///   println!("{:?}", response);
/// }
pub async fn request<T>(url: String, token: &String, client: &Client) -> Result<T, BbError> 
where 
    T: DeserializeOwned,
{
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

/// Function to request the user's profile from the Spotify API
/// 
/// * `token` - The user's Spotify access token
/// * `client` - The reqwest client
/// 
/// # Returns
/// 
/// The user's profile
/// 
/// # Errors
/// 
/// If the request fails or the response cannot be deserialized
/// 
/// # Example
/// 
/// ```
/// use reqwest::Client;
/// 
/// use crate::util::spotify::util::request_user_profile;
/// 
/// #[tokio::main]
/// async fn main() {
///   let client = Client::new();
///   let token = "BQD1".to_string();
///   let response = request_user_profile(token, &client).await.unwrap();
/// 
///   println!("{:?}", response);
/// }
pub async fn request_user_profile(token: String, client: &Client) -> Result<SpotifyUser, BbError> {
    request::<SpotifyUser>("https://api.spotify.com/v1/me".to_string(), &token, client).await
}