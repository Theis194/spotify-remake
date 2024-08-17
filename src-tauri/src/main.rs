// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use dotenv::dotenv;
use std::env;
use chrono::prelude::*;
use serde::Serialize;
use thiserror::Error;

use crate::util::{
    spotify::{
        get_authorization_url,
        exchange_code_for_token,
        AuthResponse
    },
    config::Config
};

// Tauri does not like returnng erros that are not serializable
#[derive(Error, Debug, Serialize)]
pub enum MyError {
    #[error("Environment variable {0} not found")]
    EnvVarNotFound(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Unknown error")]
    Unknown,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn authorize() -> String {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let redirect_uri = "http://localhost:1420/";
    let auth_url = get_authorization_url(&client_id, redirect_uri);

    auth_url
}

#[tauri::command]
async fn exchange_code(code: &str) -> Result<(), MyError> {
    let _ = Config::new()
        .set_filename("config".to_string())
        .read()
        .expect("Failed to read config")
        .set("auth_code".to_string(), code.to_string())
        .write()
        .expect("Failed to write config");

    let mut config = Config::new()
        .set_filename("config".to_string())
        .read()
        .expect("Failed to read config");

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found");
    let code_verifier = config.get("code_verifier").unwrap();
    let redirect_uri = "http://localhost:1420/";

    let auth_token: AuthResponse = exchange_code_for_token(&client_id, code, redirect_uri, &code_verifier).await.unwrap();

    let auth_token_expires = Utc::now() + chrono::Duration::seconds(auth_token.expires_in);

    config
        .set("auth_token".to_string(), auth_token.access_token)
        .set("auth_token_type".to_string(), auth_token.token_type)
        .set("auth_token_scope".to_string(), auth_token.scope)
        .set("auth_token_expires".to_string(), auth_token_expires.to_rfc3339())
        .set("auth_token_refresh".to_string(), auth_token.refresh_token.unwrap_or("".to_string()))
        .write()
        .expect("Failed to write config");

    Ok(())
}

#[tauri::command]
fn current_search(current: &str) {
    println!("Current search: {}", current);
}

fn main() {
    let _ = Config::new()
        .try_read("config".to_string()).expect("Failed to read config")
        .set_if_not_exists("auth_key".to_string(), "".to_string())
        .set_if_not_exists("auth_key_expire".to_string(), "".to_string())
        .write();

    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test, authorize, exchange_code, current_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
