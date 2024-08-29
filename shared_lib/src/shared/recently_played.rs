use serde::{Serialize, Deserialize};

// Spotify User object
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct RecentlyPlayed {
    pub track_name: String,
    pub artists: String,
    pub duration_ms: i32,
    pub image_url: String,
}