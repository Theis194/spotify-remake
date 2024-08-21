use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ExternalIds { // External IDs for the user
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}