use serde::{Serialize, Deserialize};

use super::external_urls::ExternalUrls;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Context {
    pub r#type: String,
    pub href: String,
    pub external_urls: ExternalUrls,
    pub uri: String
}