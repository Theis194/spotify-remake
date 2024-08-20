use chrono::prelude::*;
use tokio::sync::Mutex;
use tauri::State;
use std::env;

use crate::{util::{
    config::{
        Config,
        Value,
    }, spotify::{
        auth::{
            exchange_code_for_token, get_authorization_url, AuthResponse
        },
            util::request_user_profile,
    }, spotify_bb_error::BbError
}, AppData};

// This function is called when the app is first started
// It checks if the user is authorized and returns a boolean
#[tauri::command]
pub fn is_user_authorized() -> bool {
    let config = Config::new()
        .try_read("cache".to_string()).expect("Failed to read config");

    let user_is_authorized = match config.get("auth_token") {
        Some(Value::AuthResponse(auth_token)) => {
            !auth_token.access_token.is_empty()
        }
        _ => false
    };

    user_is_authorized
}

// This function is called when the user clicks the authorize button
#[tauri::command]
pub fn authorize() -> String {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let redirect_uri = "http://localhost:1420/";
    let auth_url = get_authorization_url(&client_id, redirect_uri);

    auth_url
}

// This function is called when the user is redirected back to the app with the auth code
#[tauri::command]
pub async fn exchange_code(code: &str, appdata: State<'_, Mutex<AppData>>) -> Result<(), BbError> {
    // Load the config, should be present due to setup, and set the auth code
    let mut config = Config::new()
        .set_filename("cache".to_string())
        .read()
        .expect("Failed to read config")
        .set("auth_code".to_string(), Value::String(code.to_string()))
        .write()
        .expect("Failed to write config");

    // Get the code verifier from cache and the client id from the environment
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let code_verifier = config.get("code_verifier").expect("code_verifier has no value").get_string().expect("Failed to get code_verifier as string");
    let redirect_uri = "http://localhost:1420/";

    // Get the reqwest client from the app data
    let client = &appdata.lock().await.reqwest_client;

    // Exchange the auth code for an auth token
    let auth_token: AuthResponse = exchange_code_for_token(&client_id, code, redirect_uri, &code_verifier, client).await.unwrap();
    
    // Create a DateTime object for the expiration date of the auth token
    let auth_token_expires = Utc::now() + chrono::Duration::seconds(3600);

    // Set the auth token in cache
    config
        .set("auth_token".to_string(), Value::AuthResponse(auth_token))
        .write()
        .expect("Failed to write config");

    // Get the auth token from the cache
    let token = config.get("auth_token").expect("auth_token has no value").get_auth_response().expect("Failed to get auth_token as AuthResponse").access_token.clone();
    
    // Get the user profile from the Spotify API
    let spotify_user = request_user_profile(token, client).await.expect("Failed to get user profile");

    // Set the user profile and the expiration date of the auth token in cache
    config
        .set("user_profile".to_string(), Value::SpotifyUser(spotify_user))
        .set("auth_token_expires".to_string(), Value::Date(auth_token_expires))
        .write()
        .expect("Failed to write config");

    Ok(())
}