use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CoreError {
    NotFound(String),
    AlreadyExists(String),
    NotImplemented(String),
}
