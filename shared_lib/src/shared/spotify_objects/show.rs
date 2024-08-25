use serde::{Deserialize, Serialize};

use crate::shared::spotify_object_components::{copyrights::Copyrights, external_urls::ExternalUrls};

use super::image::Image;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Show {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyrights>,
    pub description: String,
    pub html_description: String,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    pub r#type: String,
    pub uri: String,
    pub total_episodes: i32
}