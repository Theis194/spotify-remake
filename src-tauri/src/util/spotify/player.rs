use reqwest::Client;
use shared_lib::shared::spotify_objects::player::SpotifyPlayer;
use super::util::request;

use crate::util::config::Config;

pub async fn get_playback_state(client: &Client) -> SpotifyPlayer {
    let url = "https://api.spotify.com/v1/me/player".to_string();

    let token = Config::new()
        .set_filename("cache".to_string())
        .read()
        .expect("Failed to read cache")
        .get("auth_token")
        .expect("auth_token has no value")
        .get_auth_response()
        .expect("Could not find AuthResponse")
        .access_token
        .clone();
        

    let player: SpotifyPlayer = request::<SpotifyPlayer>(url, &token, client).await.expect("Requesting spotify player failed");

    player
}