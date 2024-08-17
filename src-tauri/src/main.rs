// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;

use dotenv::dotenv;
use std::env;

use crate::util::{
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
        .invoke_handler(tauri::generate_handler![greet, test, current_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
