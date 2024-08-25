use serde::{Serialize, Deserialize};

use crate::shared::spotify_object_components::{action::Action, context::Context, device::Device, item::Item};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct SpotifyPlayer {
    pub device: Device,
    pub repeat_state: String,
    pub shuffle_state: bool,
    pub context: Context,
    pub timestamp: i64,
    pub progress_ns: Option<i64>,
    pub is_playing: bool,
    pub item: Option<Item>,
    pub currently_playing_type: String,
    pub actions: Action
}