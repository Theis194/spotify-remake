use serde::{Serialize, Deserialize};

// Spotify User object
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Image { // The user's profile image
    pub height: Option<i32>,
    pub url: String,
    pub width: Option<i32>,
}