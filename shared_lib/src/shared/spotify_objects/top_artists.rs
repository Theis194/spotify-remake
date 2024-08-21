use serde::{Serialize, Deserialize};

use super::artist::Artist;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TopArtists {
    pub href: String,
    pub limit: i32,
    pub next: Option<String>,
    pub offset: i32,
    pub previous: Option<String>,
    pub total: i32,
    pub items: Vec<Artist>,
}