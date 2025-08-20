use crate::common::{SafeU64, debug_output};
use minidump::{MinidumpMemoryInfoList, UnifiedMemoryList};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Serialize)]
pub struct MemoryData {
    pub regions: Vec<MemoryRegion>,
    pub regions_count: usize,
    pub memory_info: Option<MemoryRangeMap>,
    pub has_memory_info_stream: bool,
    pub total_memory_size: u64,
    pub total_memory_size_formatted: String,
    pub debug: Option<String>,
}

#[derive(Serialize)]
pub struct MemoryRegion {
    pub start_address: SafeU64,
    pub end_address: SafeU64,
    pub size: u64,
    pub size_formatted: String,
    pub has_data: bool,
    pub data_size: usize,
    pub address_range: String,
}

#[derive(Serialize)]
pub struct MemoryRangeMap {
    pub ranges: Vec<MemoryInfoRange>,
    pub ranges_count: usize,
}

#[derive(Serialize)]
pub struct MemoryInfoRange {
    pub base_address: SafeU64,
    pub allocation_base: SafeU64,
    pub region_size: u64,
    pub region_size_formatted: String,
    pub state: String,
    pub state_value: u32,
    pub protection: String,
    pub protection_value: u32,
    pub allocation_protection: String,
    pub allocation_protection_value: u32,
    pub memory_type: String,
    pub memory_type_value: u32,
}

pub fn parse_memory_data(memory: &UnifiedMemoryList) -> MemoryData {
    let mut regions = Vec::new();

    // Parse memory regions from the memory list
    for memory_region in memory.iter() {
        let start_addr = memory_region.base_address();
        let size = memory_region.size();
        let end_addr = start_addr + size;
        let bytes = memory_region.bytes();
        let data_size = bytes.len();
        let has_data = !bytes.is_empty();

        let start_address = start_addr.into();
        let end_address = end_addr.into();
        let size_formatted = format_memory_size(size);
        let address_range = format!("{:#x} - {:#x}", start_addr, end_addr);

        regions.push(MemoryRegion {
            start_address,
            end_address,
            size,
            size_formatted,
            has_data,
            data_size,
            address_range,
        });
    }

    // Sort memory regions by start address to ensure consistent ordering
    // from lowest to highest address
    regions.sort_by_key(|region| region.start_address.raw_value());

    let regions_count = regions.len();

    // Calculate total memory size
    let total_memory_size: u64 = regions.iter().map(|r| r.size).sum();
    let total_memory_size_formatted = format_memory_size(total_memory_size);

    MemoryData {
        regions,
        regions_count,
        memory_info: None,             // Will be populated separately if available
        has_memory_info_stream: false, // Will be set when memory info is available
        total_memory_size,
        total_memory_size_formatted,
        debug: debug_output(memory),
    }
}

pub fn parse_memory_info_data(memory_info: &MinidumpMemoryInfoList) -> MemoryRangeMap {
    let mut ranges = Vec::new();

    for info in memory_info.iter() {
        let base_address = info.raw.base_address.into();
        let allocation_base = info.raw.allocation_base.into();
        let region_size = info.raw.region_size;
        let region_size_formatted = format_memory_size(region_size);

        // Parse memory state - convert enum to u32
        let state_value = info.state.bits();
        let (state, _) = parse_memory_state(state_value);

        // Parse memory protection - convert enum to u32
        let protection_value = info.protection.bits();
        let (protection, _) = parse_memory_protection(protection_value);
        let allocation_protection_value = info.allocation_protection.bits();
        let (allocation_protection, _) = parse_memory_protection(allocation_protection_value);

        // Parse memory type - convert enum to u32
        let memory_type_value = info.ty.bits();
        let (memory_type, _) = parse_memory_type(memory_type_value);

        ranges.push(MemoryInfoRange {
            base_address,
            allocation_base,
            region_size,
            region_size_formatted,
            state,
            state_value,
            protection,
            protection_value,
            allocation_protection,
            allocation_protection_value,
            memory_type,
            memory_type_value,
        });
    }

    // Sort memory info ranges by base address to ensure consistent ordering
    // from lowest to highest address
    ranges.sort_by_key(|range| range.base_address.raw_value());

    let ranges_count = ranges.len();

    MemoryRangeMap {
        ranges,
        ranges_count,
    }
}

// Helper function to format memory size in human readable format
fn format_memory_size(bytes: u64) -> String {
    if bytes == 0 {
        return "0 bytes".to_string();
    }

    let k = 1024u64;
    let sizes = ["bytes", "KB", "MB", "GB", "TB"];
    let i = (bytes as f64).log(k as f64).floor() as usize;
    let i = i.min(sizes.len() - 1);

    if i == 0 {
        format!("{} bytes", bytes)
    } else {
        let size = bytes as f64 / (k.pow(i as u32) as f64);
        format!("{:.1} {}", size, sizes[i])
    }
}

// Optimized memory state parsing with lookup tables
static STATE_FLAGS: OnceLock<Vec<(u32, &'static str)>> = OnceLock::new();

fn get_state_flags() -> &'static Vec<(u32, &'static str)> {
    STATE_FLAGS.get_or_init(|| {
        vec![
            (0x1000, "MEM_COMMIT"),
            (0x2000, "MEM_RESERVE"),
            (0x10000, "MEM_FREE"),
        ]
    })
}

fn parse_memory_state(state: u32) -> (String, u32) {
    let state_flags: Vec<String> = get_state_flags()
        .iter()
        .filter_map(|&(flag_bit, flag_name)| {
            if state & flag_bit != 0 {
                Some(flag_name.to_string())
            } else {
                None
            }
        })
        .collect();

    let state_str = if state_flags.is_empty() {
        format!("UNKNOWN(0x{:x})", state)
    } else {
        state_flags.join(" | ")
    };

    (state_str, state)
}

// Optimized memory protection parsing with lookup tables
static PROTECTION_FLAGS: OnceLock<HashMap<u32, &'static str>> = OnceLock::new();
static PROTECTION_MODIFIERS: OnceLock<Vec<(u32, &'static str)>> = OnceLock::new();

fn get_protection_flags() -> &'static HashMap<u32, &'static str> {
    PROTECTION_FLAGS.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(0x01, "PAGE_NOACCESS");
        map.insert(0x02, "PAGE_READONLY");
        map.insert(0x04, "PAGE_READWRITE");
        map.insert(0x08, "PAGE_WRITECOPY");
        map.insert(0x10, "PAGE_EXECUTE");
        map.insert(0x20, "PAGE_EXECUTE_READ");
        map.insert(0x40, "PAGE_EXECUTE_READWRITE");
        map.insert(0x80, "PAGE_EXECUTE_WRITECOPY");
        map
    })
}

fn get_protection_modifiers() -> &'static Vec<(u32, &'static str)> {
    PROTECTION_MODIFIERS.get_or_init(|| {
        vec![
            (0x100, "PAGE_GUARD"),
            (0x200, "PAGE_NOCACHE"),
            (0x400, "PAGE_WRITECOMBINE"),
        ]
    })
}

fn parse_memory_protection(protection: u32) -> (String, u32) {
    if protection == 0 {
        return ("NONE".to_string(), 0);
    }

    let mut protection_flags = Vec::new();
    let protection_lookup = get_protection_flags();

    // Check basic protection flags
    let basic_protection = protection & 0xFF;
    if let Some(&flag_name) = protection_lookup.get(&basic_protection) {
        protection_flags.push(flag_name.to_string());
    } else {
        protection_flags.push(format!("UNKNOWN(0x{:x})", basic_protection));
    }

    // Check modifier flags
    for &(flag_bit, flag_name) in get_protection_modifiers() {
        if protection & flag_bit != 0 {
            protection_flags.push(flag_name.to_string());
        }
    }

    let protection_str = protection_flags.join(" | ");
    (protection_str, protection)
}

// Optimized memory type parsing with lookup tables
static TYPE_FLAGS: OnceLock<Vec<(u32, &'static str)>> = OnceLock::new();

fn get_type_flags() -> &'static Vec<(u32, &'static str)> {
    TYPE_FLAGS.get_or_init(|| {
        vec![
            (0x20000, "MEM_PRIVATE"),
            (0x40000, "MEM_MAPPED"),
            (0x1000000, "MEM_IMAGE"),
        ]
    })
}

fn parse_memory_type(memory_type: u32) -> (String, u32) {
    let type_flags: Vec<String> = get_type_flags()
        .iter()
        .filter_map(|&(flag_bit, flag_name)| {
            if memory_type & flag_bit != 0 {
                Some(flag_name.to_string())
            } else {
                None
            }
        })
        .collect();

    let type_str = if type_flags.is_empty() {
        format!("UNKNOWN(0x{:x})", memory_type)
    } else {
        type_flags.join(" | ")
    };

    (type_str, memory_type)
}
