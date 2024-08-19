use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
}

pub async fn exchange_code_for_token(client_id: &str, code: &str, redirect_uri: &str, code_verifier: &str) -> Result<AuthResponse, Box<dyn Error>> {
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("code_verifier", code_verifier),
    ];

    let client = Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    let auth_response: AuthResponse = response.json().await?;
    Ok(auth_response)
}

pub fn get_authorization_url(client_id: &str, redirect_uri: &str) -> String {
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);

    let _ = Config::new()
        .set_filename("cache".to_string())
        .read()
        .expect("Failed to read config")
        .set("code_verifier".to_string(), Value::String(code_verifier))
        .write();

    format!(
        "https://accounts.spotify.com/authorize?response_type=code&client_id={}&scope=user-read-private%20user-read-email&redirect_uri={}&code_challenge_method=S256&code_challenge={}",
        client_id, redirect_uri, code_challenge
    )
}

fn generate_code_verifier() -> String {
    let verifier: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    verifier
}

fn generate_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    encode_config(&hash, URL_SAFE_NO_PAD)
}

pub async fn refresh_auth_token() -> Result<(), BbError> {
    let mut config = Config::new()
        .set_filename("cache".to_string())
        .read()
        .expect("Failed to read config");

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let refresh_token = config.get("auth_token_refresh").expect("auth_token_refresh has no value").get_string().expect("Failed to get auth_token_refresh as string");

    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token.as_str()),
        ("client_id", client_id.as_str()),
    ];

    let client = Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .expect("Failed to send request");

    let auth_response: AuthResponse = response.json().await.expect("Failed to exchange code for token");

    let auth_token_expires = Utc::now() + chrono::Duration::seconds(auth_response.expires_in);

    config
        .set("auth_token".to_string(), Value::String(auth_response.access_token) )
        .set("auth_token_type".to_string(), Value::String(auth_response.token_type))
        .set("auth_token_scope".to_string(), Value::String(auth_response.scope))
        .set("auth_token_expires".to_string(), Value::Date(auth_token_expires))
        .set("auth_token_refresh".to_string(), Value::String(auth_response.refresh_token.unwrap_or("".to_string())))
        .write()
        .expect("Failed to write config");

    Ok(())
}