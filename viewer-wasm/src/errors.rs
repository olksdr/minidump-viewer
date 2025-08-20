use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize)]
pub enum ViewerError {
    MinidumpRead(String),
    Serialization(String),
    DebugInfo(String),
}

impl std::fmt::Display for ViewerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewerError::MinidumpRead(msg) => write!(f, "minidump read error: {}", msg),
            ViewerError::Serialization(msg) => write!(f, "serialization error: {}", msg),
            ViewerError::DebugInfo(msg) => write!(f, "debug info error: {}", msg),
        }
    }
}

impl std::error::Error for ViewerError {}

impl From<ViewerError> for JsValue {
    fn from(error: ViewerError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

impl From<minidump::Error> for ViewerError {
    fn from(error: minidump::Error) -> Self {
        ViewerError::MinidumpRead(format!("{:?}", error))
    }
}

impl From<serde_wasm_bindgen::Error> for ViewerError {
    fn from(error: serde_wasm_bindgen::Error) -> Self {
        ViewerError::Serialization(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ViewerError>;
