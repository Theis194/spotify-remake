use serde::{Serialize, Deserialize};

use crate::shared::spotify_object_components::external_urls::ExternalUrls;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct SimplifiedArtist {
    pub external_urls: ExternalUrls, // External URLs for the artist
    pub href: String, // A link to the Web API endpoint providing full details of the artist
    pub id: String, // The Spotify ID for the artist
    pub name: String, // The name of the artist
    pub r#type: String, // The object type: "artist"
    pub uri: String, // The Spotify URI for the artist
}