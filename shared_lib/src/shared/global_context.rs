use super::{profile_data::ProfileData, spotify_objects::track::Track};

#[derive(Clone, Debug, Default)]
pub struct GlobalContext {
    pub profile: ProfileData,
    pub profile_loaded: bool,
    pub currently_playing: Option<Track>,
    pub access_token: String,
    pub device_id: String,
    pub is_playing: bool,
}