// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use dotenv::dotenv;
use std::env;
use chrono::prelude::*;

use crate::util::{
    spotify::auth::{
        get_authorization_url,
        exchange_code_for_token,
        refresh_auth_token,
        AuthResponse
    },
    spotify_bb_error::BbError,
    config::{
        Config,
        Value,
    }
};

#[tauri::command]
fn is_user_authorized() -> bool {
    let config = Config::new()
        .try_read("cache".to_string()).expect("Failed to read config");

    let user_is_authorized = config
        .get("user_is_authorized").expect("user_is_authorized has no value")
        .get_bool().expect("Failed to get user_is_authorized as bool");

    *user_is_authorized
}

#[tauri::command]
fn authorize() -> String {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let redirect_uri = "http://localhost:1420/";
    let auth_url = get_authorization_url(&client_id, redirect_uri);

    auth_url
}

#[tauri::command]
async fn exchange_code(code: &str) -> Result<(), BbError> {
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

    let auth_token: AuthResponse = exchange_code_for_token(&client_id, code, redirect_uri, &code_verifier).await.unwrap();

    let auth_token_expires = Utc::now() + chrono::Duration::seconds(auth_token.expires_in);

    config
        .set("auth_token".to_string(), Value::String(auth_token.access_token))
        .set("auth_token_type".to_string(), Value::String(auth_token.token_type))
        .set("auth_token_scope".to_string(), Value::String(auth_token.scope))
        .set("auth_token_expires".to_string(), Value::Date(auth_token_expires))
        .set("auth_token_refresh".to_string(), Value::String(auth_token.refresh_token.unwrap_or("".to_string())))
        .set("user_is_authorized".to_string(), Value::Bool(true))
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
    
    setup().await;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![authorize, exchange_code, current_search, is_user_authorized])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


async fn setup() {
    let mut config = Config::new()
        .try_read("cache".to_string()).expect("Failed to read config")
        .set_if_not_exists("auth_token".to_string(), Value::String("".to_string()))
        .set_if_not_exists("auth_token_type".to_string(), Value::String("".to_string()))
        .set_if_not_exists("auth_token_scope".to_string(), Value::String("".to_string()))
        .set_if_not_exists("auth_token_refresh".to_string(), Value::String("".to_string()))
        .write()
        .expect("Failed to write config");

    let auth_token_refresh = config
                                        .get("auth_token_refresh")
                                        .expect("auth_token_refresh has no value")
                                        .get_string().expect("Failed to get auth_token_refresh as string");
    
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
                refresh_auth_token().await.expect("Failed to refresh auth token");
            }
        }
        None => {
            println!("No auth token expiration date found");
        }
    }
}