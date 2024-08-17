use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::env;
use chrono::prelude::*;

use rand::Rng;
use sha2::{Digest, Sha256};
use base64::{encode_config, URL_SAFE_NO_PAD};
use super::config::*;

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
}

pub async fn exchange_code_for_token(client_id: &str, client_secret: &str, code: &str, redirect_uri: &str, code_verifier: &str) -> Result<AuthResponse, Box<dyn Error>> {
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
        .form(&params)
        .send()
        .await?;

    let auth_response: AuthResponse = response.json().await?;
    Ok(auth_response)
}

pub fn get_authorization_url(client_id: &str, redirect_uri: &str) -> String {
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);

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

// Anything below here might be deprecated
pub async fn get_auth_key() -> Result<String, Box<dyn Error>> {
    let mut config = Config::new()
        .set_filename("config".to_string())
        .read()
        .expect("Failed to read config");

    if !(config.has("auth_key") && config.has("auth_key_expire")) {
        config = match update_auth_key(config).await {
            Ok(key) => key,
            Err(e) => { return Err(e.into());}  
        };

        return Ok(config.get("auth_key").unwrap().to_string());

        
    }

    if config.get("auth_key").unwrap() == "" || config.get("auth_key_expire").unwrap() == "" {
        config = match update_auth_key(config).await {
            Ok(key) => key,
            Err(e) => { return Err(e.into());}  
        };

        return Ok(config.get("auth_key").unwrap().to_string());
    }

    let expire_time = DateTime::parse_from_rfc3339(config.get("auth_key_expire").unwrap()).unwrap();
    let current_time = Utc::now();

    if current_time > expire_time {
        config = match update_auth_key(config).await {
            Ok(key) => key,
            Err(e) => { return Err(e.into());}  
        };
        
        return Ok(config.get("auth_key").unwrap().to_string());
    }

    return Ok(config.get("auth_key").unwrap().to_string());
}

async fn request_auth_key() -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");

    let mut form_data = HashMap::new();
    form_data.insert("grant_type", "client_credentials");
    form_data.insert("client_id", &client_id);
    form_data.insert("client_secret", &client_secret);

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&form_data)
        .send()
        .await?;

    if response.status().is_success() {             // Convert response.text into json using serde to get access_token
        let response_text: String = response.text().await?;
        let auth_response: AuthResponse = serde_json::from_str(&response_text)?;
        Ok(auth_response.access_token)
    } else {
        Err("Failed to get auth key".into())
    }
}

async fn update_auth_key(mut config: Config) -> Result<Config, Box<dyn Error>> {
    let current_time = Utc::now();
    let expire_time = current_time + chrono::Duration::minutes(60);

    let new_auth_key = match request_auth_key().await {
        Ok(key) => key,
        Err(e) => { return Err(e.into());} 
    };

    let _ = config
        .set("auth_key".to_string(), new_auth_key.clone())
        .set("auth_key_expire".to_string(), expire_time.to_rfc3339())
        .write()
        .expect("Failed to write config");
    
    Ok(config)
}