use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::env;
use chrono::prelude::*;

use rand::Rng;
use sha2::{Digest, Sha256};
use base64::{encode_config, URL_SAFE_NO_PAD};
use crate::util::{
    config::{
        Config,
        Value,
    },
    spotify_bb_error::BbError
};

// Struct to hold the response from the Spotify API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
}

impl AuthResponse {
    pub fn new() -> AuthResponse {
        AuthResponse {
            access_token: String::new(),
            token_type: String::new(),
            scope: String::new(),
            expires_in: 0,
            refresh_token: None,
        }
    }
}

// Function to exchange the authorization code for an access token
pub async fn exchange_code_for_token(client_id: &str, code: &str, redirect_uri: &str, code_verifier: &str, client: &Client) -> Result<AuthResponse, Box<dyn Error>> {
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("code_verifier", code_verifier),
    ];

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    let auth_response: AuthResponse = response.json().await?;
    Ok(auth_response)
}

// Function to get the authorization URL
pub fn get_authorization_url(client_id: &str, redirect_uri: &str) -> String {
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);

    let _ = Config::new()
        .set_filename("cache".to_string())
        .read()
        .expect("Failed to read config")
        .set("code_verifier".to_string(), Value::String(code_verifier))
        .write();

    let scopes = "user-read-private%20user-read-email%20user-top-read%20playlist-read-private";

    format!(
        "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope={}&redirect_uri={}&code_challenge_method=S256&code_challenge={}",
        client_id, scopes, redirect_uri, code_challenge
    )
}

// Function to generate a code verifier
fn generate_code_verifier() -> String {
    let verifier: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    verifier
}

// Function to generate a code challenge
fn generate_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    encode_config(&hash, URL_SAFE_NO_PAD)
}

// Function to refresh the user's access token
pub async fn refresh_auth_token(client: &Client) -> Result<(), BbError> {
    let mut config = Config::new()
        .set_filename("cache".to_string())
        .read()
        .expect("Failed to read config");

    // Get the client id from enviornment variables and the refresh token from the config
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let refresh_token = config.get("auth_token").expect("auth_token has no value").get_auth_response().unwrap().refresh_token.clone().expect("refresh_token has no value");

    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token.as_str()),
        ("client_id", client_id.as_str()),
    ];
    
    // Send the request to the Spotify API
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .expect("Failed to send request");

    // Deserialize the response
    let auth_response: AuthResponse = response.json().await.expect("Failed to exchange code for token");

    let auth_token_expires = Utc::now() + chrono::Duration::seconds(3600);

    // Set the new auth token and expiration in the config
    config
        .set("auth_token".to_string(), Value::AuthResponse(auth_response.clone()))
        .set("auth_token_expires".to_string(), Value::Date(auth_token_expires))
        .write()
        .expect("Failed to write config");

    Ok(())
}