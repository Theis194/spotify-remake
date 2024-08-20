use serde::{Serialize, Deserialize};

// Spotify User object
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct SpotifyUser {
    pub country: String, // The country of the user
    pub display_name: String, // The name displayed on the user's profile
    pub email: String, // The user's email address
    pub explicit_content: ExplicitContent, // The user's explicit content settings
    pub external_urls: ExternalUrls, // External URLs for the user
    pub followers: Followers, // Information about the user's followers
    pub href: String, // A link to the Web API endpoint for the user
    pub id: String, // The Spotify user ID for the user
    pub images: Vec<Image>, // The user's profile image
    pub product: String, // The user's Spotify subscription level
    pub r#type: String, // The object type: "user"
    pub uri: String, // The Spotify URI for the user
}

// User object sub-objects
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ExplicitContent { // The user's explicit content settings
    pub filter_enabled: bool,
    pub filter_locked: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ExternalUrls { // External URLs for the user
    pub spotify: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Followers { // Information about the user's followers
    pub href: Option<String>,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Image { // The user's profile image
    pub height: Option<i32>,
    pub url: String,
    pub width: Option<i32>,
}