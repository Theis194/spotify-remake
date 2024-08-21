use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Restrictions { // Restrictions for the user
    pub reason: String,
}