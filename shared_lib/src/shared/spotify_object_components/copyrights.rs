use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Copyrights {
    pub text: String,
    pub r#type: String
}