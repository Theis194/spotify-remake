use serde::{Deserialize, Serialize};

use crate::shared::spotify_object_components::{external_urls::ExternalUrls, restrictions::Restrictions, resume_point::ResumePoint};

use super::{image::Image, show::Show};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Episode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    pub html: String,
    pub duration_ms: i64,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub resume_point: Option<ResumePoint>,
    pub r#type: String,
    pub uri: String,
    pub restrictions: Restrictions,
    pub show: Show
}