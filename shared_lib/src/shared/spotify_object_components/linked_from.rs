use serde::{Serialize, Deserialize};

use super::external_urls::ExternalUrls;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct LinkedFrom { // Linked from object
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub r#type: String,
    pub uri: String,
}