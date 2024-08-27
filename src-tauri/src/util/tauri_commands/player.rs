use crate::util::config::{
    Config, 
    Value
};

#[tauri::command(rename_all = "snake_case")]
pub fn set_device_id(device_id: String) {
    println!("Setting device id");

    Config::new()
        .set_filename("cache".to_string())
        .read()
        .expect("failed to read cache")
        .set("player_device_id".to_string(), Value::String(device_id))
        .write()
        .expect("Failed to write cache");
}