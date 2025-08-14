use serde::Serialize;

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

// Helper function to format u64 as hex string
pub fn format_hex_u64(value: u64) -> String {
    format!("0x{:x}", value)
}
