use crate::common::{SafeU64, debug_output};
use minidump::MinidumpModuleList;
use serde::Serialize;

#[derive(Serialize)]
pub struct ModuleData {
    pub modules: Vec<ModuleInfo>,
    pub modules_count: usize,
    pub debug: Option<String>,
}

#[derive(Serialize)]
pub struct ModuleInfo {
    pub name: String,
    pub base_of_image: String, // Formatted as hex
    pub size_of_image: u32,
    pub checksum: u32,
    pub time_date_stamp: u32,
    pub version_info: Option<VersionInfo>,
    pub cv_record_info: Option<CodeViewInfo>,
    pub misc_record_present: bool,
}

#[derive(Serialize)]
pub struct VersionInfo {
    pub file_version: Option<String>,
    pub product_version: Option<String>,
    pub file_flags: Option<Vec<String>>,
    pub file_type: Option<String>,
    pub file_os: Option<String>,
}

#[derive(Serialize)]
pub struct CodeViewInfo {
    pub format: String,
    pub identifier: Option<String>,
    pub age: Option<u32>,
    pub pdb_filename: Option<String>,
}

pub fn parse_modules_data(modules: &MinidumpModuleList) -> ModuleData {
    let parsed_modules = modules
        .iter()
        .map(|module| {
            let name = module.name.clone();
            let base_of_image = SafeU64::from(module.raw.base_of_image)
                .to_hex_string()
                .to_string();
            let raw = &module.raw;

            ModuleInfo {
                name,
                base_of_image,
                size_of_image: raw.size_of_image,
                checksum: raw.checksum,
                time_date_stamp: raw.time_date_stamp,
                version_info: parse_version_info(&raw.version_info),
                cv_record_info: module.codeview_info.as_ref().and_then(parse_codeview_info),
                misc_record_present: raw.misc_record.data_size > 0,
            }
        })
        .collect();

    ModuleData {
        modules: parsed_modules,
        modules_count: modules.iter().count(),
        debug: debug_output(modules),
    }
}

// Helper function to format version from high and low parts
fn format_version(version_hi: u32, version_lo: u32) -> Option<String> {
    if version_hi != 0 || version_lo != 0 {
        let major = (version_hi >> 16) & 0xFFFF;
        let minor = version_hi & 0xFFFF;
        let build = (version_lo >> 16) & 0xFFFF;
        let revision = version_lo & 0xFFFF;
        Some(format!("{}.{}.{}.{}", major, minor, build, revision))
    } else {
        None
    }
}

// For now, create a simple version info parser
fn parse_version_info(version_info: &minidump::format::VS_FIXEDFILEINFO) -> Option<VersionInfo> {
    // Check if version info is valid (signature should be 0xFEEF04BD)
    if version_info.signature != 0xFEEF04BD {
        return None;
    }

    let file_version = format_version(version_info.file_version_hi, version_info.file_version_lo);
    let product_version = format_version(
        version_info.product_version_hi,
        version_info.product_version_lo,
    );

    // Parse file flags
    let file_flags = parse_file_flags(version_info.file_flags);

    // Parse file type
    let file_type = parse_file_type(version_info.file_type);

    // Parse file OS
    let file_os = parse_file_os(version_info.file_os);

    Some(VersionInfo {
        file_version,
        product_version,
        file_flags: if file_flags.is_empty() {
            None
        } else {
            Some(file_flags)
        },
        file_type,
        file_os,
    })
}

fn parse_file_flags(flags: u32) -> Vec<String> {
    let mut result = Vec::new();

    if flags & 0x01 != 0 {
        result.push("DEBUG".to_string());
    }
    if flags & 0x10 != 0 {
        result.push("INFOINFERRED".to_string());
    }
    if flags & 0x04 != 0 {
        result.push("PATCHED".to_string());
    }
    if flags & 0x02 != 0 {
        result.push("PRERELEASE".to_string());
    }
    if flags & 0x08 != 0 {
        result.push("PRIVATEBUILD".to_string());
    }
    if flags & 0x20 != 0 {
        result.push("SPECIALBUILD".to_string());
    }

    result
}

fn parse_file_type(file_type: u32) -> Option<String> {
    match file_type {
        0x01 => Some("APPLICATION".to_string()),
        0x02 => Some("DLL".to_string()),
        0x03 => Some("DRIVER".to_string()),
        0x04 => Some("FONT".to_string()),
        0x05 => Some("VXD".to_string()),
        0x07 => Some("STATIC_LIB".to_string()),
        _ => None,
    }
}

fn parse_file_os(file_os: u32) -> Option<String> {
    match file_os {
        0x00040004 => Some("WIN32".to_string()),
        0x00040000 => Some("WIN16".to_string()),
        0x00010000 => Some("DOS".to_string()),
        0x00020000 => Some("OS216".to_string()),
        0x00030000 => Some("OS232".to_string()),
        0x00040001 => Some("NT".to_string()),
        _ => None,
    }
}

fn parse_codeview_info(cv: &minidump::CodeView) -> Option<CodeViewInfo> {
    match cv {
        minidump::CodeView::Pdb70(pdb70) => {
            // Convert Vec<u8> to String for PDB filename
            let pdb_filename = if pdb70.pdb_file_name.is_empty() {
                None
            } else {
                String::from_utf8(pdb70.pdb_file_name.clone()).ok()
            };

            Some(CodeViewInfo {
                format: "PDB70".to_string(),
                identifier: Some(format!("{}", pdb70.signature)),
                age: Some(pdb70.age),
                pdb_filename,
            })
        }
        minidump::CodeView::Pdb20(pdb20) => {
            // Convert Vec<u8> to String for PDB filename
            let pdb_filename = if pdb20.pdb_file_name.is_empty() {
                None
            } else {
                String::from_utf8(pdb20.pdb_file_name.clone()).ok()
            };

            Some(CodeViewInfo {
                format: "PDB20".to_string(),
                identifier: Some(format!("{:08x}{:08x}", pdb20.signature, pdb20.age)),
                age: Some(pdb20.age),
                pdb_filename,
            })
        }
        minidump::CodeView::Elf(elf) => {
            // Format Vec<u8> as hex string for build ID
            let build_id_hex = elf
                .build_id
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();

            Some(CodeViewInfo {
                format: "ELF".to_string(),
                identifier: Some(build_id_hex),
                age: None,
                pdb_filename: None,
            })
        }
        _ => Some(CodeViewInfo {
            format: "Unknown".to_string(),
            identifier: None,
            age: None,
            pdb_filename: None,
        }),
    }
}

pub fn get_modules_count(modules: &MinidumpModuleList) -> usize {
    modules.iter().count()
}
