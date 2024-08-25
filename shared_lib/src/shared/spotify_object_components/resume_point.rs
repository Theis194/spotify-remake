use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ResumePoint {
    pub fully_played: bool,
    pub resume_position_ms: i64
}