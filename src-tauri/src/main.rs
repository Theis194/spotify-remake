// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use std::env;

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
fn getSpotifyClient() -> String {
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
    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test, getSpotifyClient, getSpotifySecret, current_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
