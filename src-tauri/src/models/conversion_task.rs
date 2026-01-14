use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionTask {
    pub id: String,
    pub input_path: String,
    pub output_path: String,
    pub from_format: String,
    pub to_format: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub id: String,
    pub success: bool,
    pub message: String,
    pub output_path: Option<String>,
}
