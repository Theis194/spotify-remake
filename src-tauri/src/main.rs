// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use dotenv::dotenv;
use std::env;

use crate::util::{
    spotify::get_auth_key, 
    config::Config
};

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
async fn getSpotifyClient() -> String {
    let auth_key = get_auth_key().await.unwrap();

    println!("Auth key: {}", auth_key);
    println!("Getting Spotify token");
    let token = env::var("CLIENT_ID").expect("CLIENT_ID must be set");

    token
}

#[tauri::command]
fn getSpotifySecret() -> String {
    println!("Getting Spotify token");
    let token = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");

    token
}

#[tauri::command]
fn current_search(current: &str) {
    println!("Current search: {}", current);
}

fn main() {
    let _ = Config::new()
        .set_filename("config".to_string())
        .set_if_not_exists("auth_key".to_string(), "".to_string())
        .set_if_not_exists("auth_key_expire".to_string(), "".to_string())
        .write();

    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test, getSpotifyClient, getSpotifySecret, current_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
