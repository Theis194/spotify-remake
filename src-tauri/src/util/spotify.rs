use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::env;
use chrono::prelude::*;

use super::config::*;

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
}

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

async fn get_new_auth_key() -> Result<String, Box<dyn Error>> {
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

    let new_auth_key = match get_new_auth_key().await {
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