// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use chrono::prelude::*;
use async_std::task;
use tokio::sync::Mutex;
use reqwest::Client;
use dotenv::dotenv;
use tauri::Manager;
use std::env;

use crate::util::{
    spotify::auth::{
        refresh_auth_token,
        AuthResponse
    },
    config::{
        Config,
        Value,
    }
};

// Struct to hold non-mutable app data
struct AppData {
    reqwest_client: Client,
}

// Tauri example command
#[tauri::command]
fn current_search(current: &str) {
    println!("Current search: {}", current);
}

// This launches the Tauri application
// It sets up the app data and runs setup
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    setup().await;

    tauri::Builder::default()
        .setup(|app| {
            // "Global" app data
            app.manage(Mutex::new(AppData {
                reqwest_client: Client::new(),
            }));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            util::tauri_commands::auth::authorize, 
            util::tauri_commands::auth::exchange_code, 
            util::tauri_commands::auth::is_user_authorized,
            util::tauri_commands::profile::get_user_profile,
            util::tauri_commands::profile::get_profile_data,
            current_search
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

// This function is called when the app is first started
// It checks if the user is authorized and if the auth token is expired
// If the token is expired, it refreshes it
async fn setup() {
    // Load the config, if it doesn't exist, create it
    let mut config = Config::new()
        .try_read("cache".to_string()).expect("Failed to read config")
        .set_if_not_exists("auth_token".to_string(), Value::String("".to_string()))
        .set_if_not_exists("last_request_top_tracks".to_string(), Value::Date(Utc::now() - chrono::Duration::days(1)))
        .set_if_not_exists("last_request_top_artists".to_string(), Value::Date(Utc::now() - chrono::Duration::days(1)))
        .write()
        .expect("Failed to write config");

    // Get the refresh token from the config
    let auth_token_refresh = config
        .get("auth_token")
        .expect("auth_token has no value")
        .get_auth_response()
        .unwrap_or(&AuthResponse::new())
        .refresh_token
        .clone()
        .unwrap_or("".to_string());
                                        
    // If the refresh token is empty, the user is not authorized
    if auth_token_refresh.is_empty() {
        config
            .set("user_is_authorized".to_string(), Value::Bool(false))
            .write()
            .expect("Failed to write config");
    }

    // Check if the auth_token_expires exists in the config
    let auth_token_expires;
    if config.has("auth_token_expires") {
        auth_token_expires = config
            .get("auth_token_expires")
            .expect("auth_token_expires has no value")
            .get_date()
    } else {
        auth_token_expires = None;
    }
    
    // If the auth token has an expiration date, check if it's expired
    // If it is, refresh the token
    match auth_token_expires {
        Some(auth_token_expires) => {
            let now = Utc::now();

            if now > *auth_token_expires {
                println!("Token expired, refreshing...");
                refresh_auth_token().await.expect("Failed to refresh auth token");
                println!("Token refreshed");
            }
        }
        None => {
            println!("No auth token expiration date found");
        }
    }
}