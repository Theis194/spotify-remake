use serde::{Serialize, Deserialize};

// Spotify User object
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Followers { // Information about the user's followers
    pub href: Option<String>,
    pub total: i32,
}