use serde::Serialize;
use std::fmt::Debug;

// For consistency in minidump viewing, all u64 values are displayed as hex strings
pub struct SafeU64 {
    hex_string: String,
    raw_value: u64,
}

impl SafeU64 {
    /// Get the raw u64 value for sorting and comparison purposes
    pub fn raw_value(&self) -> u64 {
        self.raw_value
    }

    /// Get the hex string representation
    pub fn to_hex_string(&self) -> &str {
        &self.hex_string
    }
}

impl From<u64> for SafeU64 {
    fn from(value: u64) -> Self {
        SafeU64 {
            hex_string: format!("0x{:x}", value),
            raw_value: value,
        }
    }
}

impl Serialize for SafeU64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.hex_string.serialize(serializer)
    }
}

// Common traits for consistent parsing patterns

/// Trait for objects that can provide debug serialization
pub trait DebugSerializable {
    fn debug_string(&self) -> String;
}

/// Blanket implementation for any Debug type
impl<T: Debug> DebugSerializable for T {
    fn debug_string(&self) -> String {
        format!("{:#?}", self)
    }
}

/// Helper function for consistent debug output patterns
pub fn debug_output<T: Debug>(item: &T) -> Option<String> {
    Some(item.debug_string())
}
