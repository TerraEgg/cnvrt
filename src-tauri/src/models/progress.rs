use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdate {
    pub id: String,
    pub percentage: u32,
    pub message: String,
}
