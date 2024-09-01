use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TrackInfo {
    pub artists: String,
    pub name: String,
    pub album: String,
    pub duration: i32,
    pub image: String,
    pub uri: String,
    pub position: i32,
    pub paused: bool,
    pub shuffle: bool,
    pub timestamp: i64,
}