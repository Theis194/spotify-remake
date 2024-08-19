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

struct AppData {
    reqwest_client: Client,
}

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

#[tauri::command]
fn authorize() -> String {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let redirect_uri = "http://localhost:1420/";
    let auth_url = get_authorization_url(&client_id, redirect_uri);

    auth_url
}

#[tauri::command]
async fn exchange_code(code: &str, appdata: State<'_, Mutex<AppData>>) -> Result<(), BbError> {
    let mut config = Config::new()
        .set_filename("cache".to_string())
        .read()
        .expect("Failed to read config")
        .set("auth_code".to_string(), Value::String(code.to_string()))
        .write()
        .expect("Failed to write config");

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let code_verifier = config.get("code_verifier").expect("code_verifier has no value").get_string().expect("Failed to get code_verifier as string");
    let redirect_uri = "http://localhost:1420/";

    let client = &appdata.lock().await.reqwest_client;

    let auth_token: AuthResponse = exchange_code_for_token(&client_id, code, redirect_uri, &code_verifier, client).await.unwrap();
    
    let auth_token_expires = Utc::now() + chrono::Duration::seconds(3600);

    config
        .set("auth_token".to_string(), Value::AuthResponse(auth_token))
        .write()
        .expect("Failed to write config");

    let token = config.get("auth_token").expect("auth_token has no value").get_auth_response().expect("Failed to get auth_token as AuthResponse").access_token.clone();
    
    let spotify_user = request_user_profile(token, client).await.expect("Failed to get user profile");

    config
        .set("user_profile".to_string(), Value::SpotifyUser(spotify_user))
        .set("auth_token_expires".to_string(), Value::Date(auth_token_expires))
        .write()
        .expect("Failed to write config");

    Ok(())
}

#[tauri::command]
fn current_search(current: &str) {
    println!("Current search: {}", current);
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData {
                reqwest_client: Client::new(),
            }));
            let app_handle = app.app_handle();
            let setup = async move {
                let client = app_handle
                    .state::<Mutex<AppData>>()
                    .lock()
                    .await
                    .reqwest_client
                    .clone();
                setup(client).await;
            };

            task::spawn(setup);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![authorize, exchange_code, current_search, is_user_authorized])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


async fn setup(client: Client) {
    let mut config = Config::new()
        .try_read("cache".to_string()).expect("Failed to read config")
        .set_if_not_exists("auth_token".to_string(), Value::String("".to_string()))
        .write()
        .expect("Failed to write config");

    let auth_token_refresh = config
        .get("auth_token")
        .expect("auth_token has no value")
        .get_auth_response()
        .unwrap_or(&AuthResponse::new())
        .refresh_token
        .clone()
        .unwrap_or("".to_string());
                                        
    
    if auth_token_refresh.is_empty() {
        config
        .set("user_is_authorized".to_string(), Value::Bool(false))
        .write()
        .expect("Failed to write config");
    }

    
    let auth_token_expires;

    if config.has("auth_token_expires") {
        auth_token_expires = config
            .get("auth_token_expires")
            .expect("auth_token_expires has no value")
            .get_date()
    } else {
        auth_token_expires = None;
    }
    
    match auth_token_expires {
        Some(auth_token_expires) => {
            let now = Utc::now();

            if now > *auth_token_expires {
                println!("Token expired, refreshing...");
                refresh_auth_token(client).await.expect("Failed to refresh auth token");
            }
        }
        None => {
            println!("No auth token expiration date found");
        }
    }
}