use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpotifyUser {
    country: String, // The country of the user
    display_name: String, // The name displayed on the user's profile
    email: String, // The user's email address
    explicit_content: ExplicitContent, // The user's explicit content settings
    external_urls: ExternalUrls, // External URLs for the user
    followers: Followers, // Information about the user's followers
    href: String, // A link to the Web API endpoint for the user
    id: String, // The Spotify user ID for the user
    images: Vec<Image>, // The user's profile image
    product: String, // The user's Spotify subscription level
    r#type: String, // The object type: "user"
    uri: String, // The Spotify URI for the user
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExplicitContent { // The user's explicit content settings
    filter_enabled: bool,
    filter_locked: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalUrls { // External URLs for the user
    spotify: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Followers { // Information about the user's followers
    href: Option<String>,
    total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image { // The user's profile image
    height: Option<i32>,
    url: String,
    width: Option<i32>,
}