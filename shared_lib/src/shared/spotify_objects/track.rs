use serde::{Serialize, Deserialize};

use crate::shared::spotify_object_components::{
    external_ids::ExternalIds, 
    external_urls::ExternalUrls, 
    linked_from::LinkedFrom, 
    restrictions::Restrictions
};

use super::{
    album::Album, 
    artist::Artist
};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Track {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i32,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub linked_from: Option<LinkedFrom>,
    pub restrictions: Option<Restrictions>,
    pub name: String,
    pub popularity: i32,
    pub preview_url: Option<String>,
    pub r#type: String,
    pub uri: String,
    pub is_local: bool,
}