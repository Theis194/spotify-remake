use serde::{Deserialize, Serialize};
use shared_lib::shared::spotify_objects::{
    top_artists::TopArtists, 
    top_tracks::TopTracks, 
    user::SpotifyUser
};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use chrono::{DateTime, Utc};

use crate::util::spotify::auth::AuthResponse;

// Config struct to hold settings
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Config {
    settings: HashMap<String, Value>,
    file_name: String,
}

// Config methods
impl Config {
    // Constructor
    pub fn new() -> Config {
        let settings = HashMap::new();
        Config { settings, file_name: String::from("") }
    }

    // Getters and setters
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.settings.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) -> Config {
        self.settings.insert(key, value);

        self.clone()
    }

    // Check if a key exists
    pub fn has(&self, key: &str) -> bool {
        self.settings.contains_key(key)
    }

    // Set a value if it doesn't exist
    pub fn set_if_not_exists(&mut self, key: String, value: Value) -> Config {
        if !self.has(&key) {
            self.set(key, value);
        }

        self.clone()
    }

    // Set the file name
    pub fn set_filename(&mut self, file_name: String) -> Config {
        self.file_name = file_name;

        self.clone()
    }

    // Reads the config file
    pub fn read(&self) -> Result<Config, Box<dyn Error>> {
        if self.file_name == "" {
            return Err("File name not set".into());
        }

        if !self.config_exists(self.file_name.clone()) {
            return Err("Config file does not exist".into());
        }
        let contents = fs::read_to_string(format!("src/config/{}.json", self.file_name)).expect("Something went wrong reading the file");
        let config: Config = serde_json::from_str(&contents).expect("Error parsing JSON");
        Ok(config)
    }

    // Tries to read the config file
    // If it doesn't exist, creates a new one
    pub fn try_read(&self, file_name: String) -> Result<Config, Box<dyn Error>> {
        if !self.config_exists(file_name.clone()) {
            return Ok(Config::new().set_filename(file_name))
        }
        let contents = fs::read_to_string(format!("src/config/{}.json", file_name)).expect("Something went wrong reading the file");
        let config: Config = serde_json::from_str(&contents).expect("Error parsing JSON");
        Ok(config)
    } 

    // Writes the config file
    pub fn write(&self) -> Result<Config, Box<dyn Error>> {
        if self.file_name == "" {
            return Err("File name not set".into());
        }

        if !self.directory_exists("src/config") {
            fs::create_dir("src/config").expect("Error creating directory");
        }

        let json = serde_json::to_string(&self).expect("Error serializing JSON");
        fs::write(format!("src/config/{}.json", self.file_name), json).expect("Error writing file");

        Ok(self.clone())
    }

    // Checks if the config file exists
    pub fn config_exists(&self, file_name: String) -> bool {
        fs::metadata(format!("src/config/{}.json", file_name)).is_ok()
    }
    
    // Checks if a directory exists
    pub fn directory_exists(&self, directory: &str) -> bool {
        match fs::metadata(directory) {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false,
        }
    } 
}

// Value enum to hold different types of values
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Value {
    String(String),
    Bool(bool),
    Date(DateTime<Utc>),
    AuthResponse(AuthResponse),
    SpotifyUser(SpotifyUser),
    TopTracks(TopTracks),
    TopArtists(TopArtists),
}

// Value methods
impl Value {
    // Getters
    pub fn get_string(&self) -> Option<&String> {
        match self {
            Value::String(value) => Some(value),
            _ => None
        }
    }

    pub fn get_bool(&self) -> Option<&bool> {
        match self {
            Value::Bool(value) => Some(value),
            _ => None
        }
    }

    pub fn get_date(&self) -> Option<&DateTime<Utc>> {
        match self {
            Value::Date(value) => Some(value),
            _ => None
        }
    }

    pub fn get_auth_response(&self) -> Option<&AuthResponse> {
        match self {
            Value::AuthResponse(value) => Some(value),
            _ => None
        }
    }

    pub fn get_spotify_user(&self) -> Option<&SpotifyUser> {
        match self {
            Value::SpotifyUser(value) => Some(value),
            _ => None
        }
    }

    pub fn get_top_tracks(&self) -> Option<&TopTracks> {
        match self {
            Value::TopTracks(value) => Some(value),
            _ => None
        }
    }

    pub fn get_top_artists(&self) -> Option<&TopArtists> {
        match self {
            Value::TopArtists(value) => Some(value),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    // Config tests

    #[test]
    fn test_set_filename() {
        let mut config = Config::new();
        config.set_filename("test".to_string());
        assert_eq!(config.file_name, "test");
    }

    #[test]
    fn test_new() {
        let config = Config::new();
        assert_eq!(config.settings.len(), 0);
    }

    #[test]
    fn test_config() {
        let mut config = Config::new();
        config.set("test".to_string(), Value::String("test".to_string()));
        assert_eq!(config.get("test").unwrap().get_string().unwrap(), "test");
    }

    #[test]
    fn test_read() {
        let filename = format!("test-{}", Uuid::new_v4());
        let mut config = Config::new();
        config.set_filename(filename.clone());
        config.set("test".to_string(), Value::String("test".to_string()));
        config.write().unwrap();
        let read_config = config.read().unwrap();
        assert_eq!(read_config.get("test").unwrap().get_string().unwrap(), "test");
        fs::remove_file(format!("src/config/{}.json", config.file_name)).expect("Error removing file");
    }

    #[test]
    fn test_try_read() {
        let filename = format!("test-{}", Uuid::new_v4());
        let mut config = Config::new();
        config.set_filename(filename.clone());
        config.set("test".to_string(), Value::String("test".to_string()));
        config.write().unwrap();
        let read_config = config.try_read(filename).unwrap();
        assert_eq!(read_config.get("test").unwrap().get_string().unwrap(), "test");
        fs::remove_file(format!("src/config/{}.json", config.file_name)).expect("Error removing file");
    }

    #[test]
    fn test_write() {
        let filename = format!("test-{}", Uuid::new_v4());
        let mut config = Config::new();
        config.set_filename(filename.clone());
        config.set("test".to_string(), Value::String("test".to_string()));
        config.write().unwrap();
        let read_config = config.read().unwrap();
        assert_eq!(read_config.get("test").unwrap().get_string().unwrap(), "test");
        fs::remove_file(format!("src/config/{}.json", config.file_name)).expect("Error removing file");
    }

    #[test]
    fn test_config_exists() {
        let filename = format!("test-{}", Uuid::new_v4());
        let mut config = Config::new();
        config.set_filename(filename.clone());
        config.write().unwrap();
        assert_eq!(config.config_exists(filename), true);
        fs::remove_file(format!("src/config/{}.json", config.file_name)).expect("Error removing file");
    }

    #[test]
    fn test_directory_exists() {
        let config = Config::new();
        assert_eq!(config.directory_exists("src/config"), true);
    }

    #[test]
    fn test_set_if_not_exists() {
        let mut config = Config::new();
        config.set_if_not_exists("test".to_string(), Value::String("test".to_string()));
        assert_eq!(config.get("test").unwrap().get_string().unwrap(), "test");
    }

    #[test]
    fn test_has() {
        let mut config = Config::new();
        config.set("test".to_string(), Value::String("test".to_string()));
        assert_eq!(config.has("test"), true);
    }

    #[test]
    fn test_set() {
        let mut config = Config::new();
        config.set("test".to_string(), Value::String("test".to_string()));
        assert_eq!(config.get("test").unwrap().get_string().unwrap(), "test");
    }

    #[test]
    fn test_get() {
        let mut config = Config::new();
        config.set("test".to_string(), Value::String("test".to_string()));
        assert_eq!(config.get("test").unwrap().get_string().unwrap(), "test");
    }

    #[test]
    fn test_read_non_existing_file() {
        let filename = format!("non_existing_file-{}", Uuid::new_v4());
        let config = Config::new().set_filename(filename.clone());
        let result = config.read();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Config file does not exist");
    }

    // Value tests

    #[test]
    fn test_value() {
        let value = Value::String("test".to_string());
        assert_eq!(value.get_string().unwrap(), "test");
    }

    #[test]
    fn test_get_string() {
        let value = Value::String("test".to_string());
        assert_eq!(value.get_string().unwrap(), "test");
    }

    #[test]
    fn test_get_bool() {
        let value = Value::Bool(true);
        assert_eq!(value.get_bool().unwrap(), &true);
    }

    #[test]
    fn test_get_date() {
        let now = Utc::now();
        let now_clone = now.clone();
        let value = Value::Date(now);
        assert_eq!(value.get_date().unwrap(), &now_clone);
    }

    #[test]
    fn test_get_auth_response() {
        let value = Value::AuthResponse(AuthResponse::new());
        assert_eq!(value.get_auth_response().unwrap(), &AuthResponse::new());
    }

    #[test]
    fn test_get_spotify_user() {
        let value = Value::SpotifyUser(SpotifyUser::default());
        assert_eq!(value.get_spotify_user().unwrap(), &SpotifyUser::default());
    }

    #[test]
    fn test_get_top_tracks() {
        let value = Value::TopTracks(TopTracks::default());
        assert_eq!(value.get_top_tracks().unwrap(), &TopTracks::default());
    }

    #[test]
    fn test_get_top_artists() {
        let value = Value::TopArtists(TopArtists::default());
        assert_eq!(value.get_top_artists().unwrap(), &TopArtists::default());
    }

    #[test]
    fn test_value_string() {
        let value = Value::String("test".to_string());
        assert_eq!(value, Value::String("test".to_string()));
    }

    #[test]
    fn test_value_bool() {
        let value = Value::Bool(true);
        assert_eq!(value, Value::Bool(true));
    }

    #[test]
    fn test_value_date() {
        let now = Utc::now();
        let value = Value::Date(now);
        assert_eq!(value, Value::Date(now));
    }

    #[test]
    fn test_value_auth_response() {
        let value = Value::AuthResponse(AuthResponse::new());
        assert_eq!(value, Value::AuthResponse(AuthResponse::new()));
    }

    #[test]
    fn test_value_spotify_user() {
        let value = Value::SpotifyUser(SpotifyUser::default());
        assert_eq!(value, Value::SpotifyUser(SpotifyUser::default()));
    }

    #[test]
    fn test_value_top_tracks() {
        let value = Value::TopTracks(TopTracks::default());
        assert_eq!(value, Value::TopTracks(TopTracks::default()));
    }

    #[test]
    fn test_value_top_artists() {
        let value = Value::TopArtists(TopArtists::default());
        assert_eq!(value, Value::TopArtists(TopArtists::default()));
    }
}