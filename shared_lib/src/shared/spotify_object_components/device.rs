use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Device {
    pub id: Option<String>,
    pub is_active: bool,
    pub is_private_session: bool,
    pub is_restricted: bool,
    pub name: String,
    pub r#type: String,
    pub volume_percent: i32,
    pub supports_volume: bool
}