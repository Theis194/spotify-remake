use serde::{Serialize, Deserialize};

use crate::shared::spotify_object_components::{
    external_urls::ExternalUrls, 
    restrictions::Restrictions
};

use super::{
    artist::Artist, 
    image::Image
};



#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Album {
    pub album_type: String,
    pub total_tracks: i32,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub restrictions: Option<Restrictions>,
    pub r#type: String,
    pub uri: String,
    pub artists: Vec<Artist>,
}