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
fn getSpotifyToken() -> String {
    println!("Getting Spotify token");
    dotenv().ok();
    let token = env::var("SPOTIFY_API_TOKEN").expect("SPOTIFY_API_TOKEN must be set");

    token
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test, getSpotifyToken])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
