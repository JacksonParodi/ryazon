use crate::ryazon::RyazonError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RyazonOutput {
    Success(String),
    Error(RyazonError),
}
