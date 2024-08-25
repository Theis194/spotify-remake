use serde::{Deserialize, Serialize};

use crate::shared::spotify_objects::{
    track::Track,
    episode::Episode
};

// Define the enum that represents the possible types for the `item` field
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")] // Customizes the enum representation
pub enum Item {
    Track(Track),    // When the item is a Track
    Episode(Episode), // When the item is an Episode
    Unknown,
}

// Implement Default for the enum
impl Default for Item {
    fn default() -> Self {
        // Specify the default variant
        Item::Unknown
    }
}