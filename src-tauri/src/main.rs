// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use chrono::prelude::*;
use async_std::task;
use tokio::sync::Mutex;
use reqwest::Client;
use dotenv::dotenv;
use tauri::{Manager, State};
use std::env;

use crate::util::{
        spotify::{
            auth::{
            get_authorization_url,
            exchange_code_for_token,
            refresh_auth_token,
            AuthResponse
        },
            util::request_user_profile,
    },
    spotify_bb_error::BbError,
    config::{
        Config,
        Value,
    }
};

// Struct to hold non-mutable app data
struct AppData {
    reqwest_client: Client,
}

// This function is called when the app is first started
// It checks if the user is authorized and returns a boolean
#[tauri::command]
fn is_user_authorized() -> bool {
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
fn authorize() -> String {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let redirect_uri = "http://localhost:1420/";
    let auth_url = get_authorization_url(&client_id, redirect_uri);

    auth_url
}

// This function is called when the user is redirected back to the app with the auth code
#[tauri::command]
async fn exchange_code(code: &str, appdata: State<'_, Mutex<AppData>>) -> Result<(), BbError> {
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

// Tauri example command
#[tauri::command]
fn current_search(current: &str) {
    println!("Current search: {}", current);
}

// This launches the Tauri application
// It sets up the app data and runs setup
#[tokio::main]
async fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            // "Global" app data
            app.manage(Mutex::new(AppData {
                reqwest_client: Client::new(),
            }));
            // Extracts the reqwest client from the app data and runs setup
            let app_handle = app.app_handle();
            let setup = async move {
                let binding = app_handle.state::<Mutex<AppData>>();

                let mut appdata = binding.lock().await;

                let client = &appdata.reqwest_client;

                setup(client).await;
            };

            // Task:.spawn is used to run the async function setup without having to await it
            task::spawn(setup);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![authorize, exchange_code, current_search, is_user_authorized])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// This function is called when the app is first started
// It checks if the user is authorized and if the auth token is expired
// If the token is expired, it refreshes it
async fn setup(client: &Client) {
    // Load the config, if it doesn't exist, create it
    let mut config = Config::new()
        .try_read("cache".to_string()).expect("Failed to read config")
        .set_if_not_exists("auth_token".to_string(), Value::String("".to_string()))
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
                refresh_auth_token(&client).await.expect("Failed to refresh auth token");
            }
        }
        None => {
            println!("No auth token expiration date found");
        }
    }
}