use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub code: u16,
    pub error: String,
    pub message: String,
}
