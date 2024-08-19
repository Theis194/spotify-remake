use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    settings: HashMap<String, Value>,
    file_name: String,
}

impl Config {
    pub fn new() -> Config {
        let settings = HashMap::new();
        Config { settings, file_name: String::from("") }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.settings.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) -> Config {
        self.settings.insert(key, value);

        self.clone()
    }

    pub fn has(&self, key: &str) -> bool {
        self.settings.contains_key(key)
    }

    pub fn set_if_not_exists(&mut self, key: String, value: Value) -> Config {
        if !self.has(&key) {
            self.set(key, value);
        }

        self.clone()
    }

    pub fn set_filename(&mut self, file_name: String) -> Config {
        self.file_name = file_name;

        self.clone()
    }

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

    pub fn try_read(&self, file_name: String) -> Result<Config, Box<dyn Error>> {
        if !self.config_exists(file_name.clone()) {
            return Ok(Config::new().set_filename(file_name))
        }
        let contents = fs::read_to_string(format!("src/config/{}.json", file_name)).expect("Something went wrong reading the file");
        let config: Config = serde_json::from_str(&contents).expect("Error parsing JSON");
        Ok(config)
    } 

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

    pub fn config_exists(&self, file_name: String) -> bool {
        fs::metadata(format!("src/config/{}.json", file_name)).is_ok()
    }
    
    pub fn directory_exists(&self, directory: &str) -> bool {
        match fs::metadata(directory) {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false,
        }
    } 
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Value {
    String(String),
    Bool(bool),
    Date(DateTime<Utc>),
}

impl Value {
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
}