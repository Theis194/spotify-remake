use serde::{Serialize, Deserialize};

use crate::shared::spotify_object_components::{
    external_urls::ExternalUrls, 
    followers::Followers
};

use super::image::Image;

// Spotify User object
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Artist {
    pub external_urls: ExternalUrls, // External URLs for the artist
    pub followers: Followers, // Information about the artist's followers
    pub genres: Vec<String>, // A list of the genres the artist is associated with
    pub href: String, // A link to the Web API endpoint providing full details of the artist
    pub id: String, // The Spotify ID for the artist
    pub images: Vec<Image>, // Images of the artist in various sizes, widest first
    pub name: String, // The name of the artist
    pub popularity: i32, // The popularity of the artist
    pub r#type: String, // The object type: "artist"
    pub uri: String, // The Spotify URI for the artist
}
