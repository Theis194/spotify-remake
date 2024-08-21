use serde::{Serialize, Deserialize};

// Spotify User object
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ExternalUrls { // External URLs for the user
    pub spotify: String,
}