use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Action {
    pub interrupting_playback: Option<bool>,
    pub pausing: Option<bool>,
    pub resuming: Option<bool>,
    pub seeking: Option<bool>,
    pub skipping_next: Option<bool>,
    pub skipping_prev: Option<bool>,
    pub toggling_repeat_context: Option<bool>,
    pub toggling_shuffle: Option<bool>,
    pub toggling_repeat_track: Option<bool>,
    pub transferring_playback: Option<bool>
}