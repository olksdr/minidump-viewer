use crate::common::debug_output;
use minidump::MinidumpSystemInfo;
use serde::Serialize;

#[derive(Serialize)]
pub struct SystemInfoRaw {
    // MINIDUMP_SYSTEM_INFO fields
    pub processor_architecture: Option<u16>,
    pub processor_level: Option<u16>,
    pub processor_revision: Option<u16>,
    pub number_of_processors: Option<u8>,
    pub product_type: Option<u8>,
    pub major_version: Option<u32>,
    pub minor_version: Option<u32>,
    pub build_number: Option<u32>,
    pub platform_id: Option<u32>,
    pub csd_version_rva: Option<u32>,
    pub suite_mask: Option<u16>,
    pub reserved2: Option<u16>,

    // Additional structured fields from debug output
    pub cpu_info_data: Option<Vec<u8>>, // CPU_INFORMATION data array
    pub os_version: Option<String>,
    pub csd_version: Option<String>,
}

#[derive(Serialize)]
pub struct SystemInfoData {
    pub os: Option<String>,
    pub cpu_info: Option<String>,
    pub raw: Option<SystemInfoRaw>,
    pub debug: Option<String>, // Raw debug output
}

pub fn parse_system_info(system: &MinidumpSystemInfo) -> SystemInfoData {
    SystemInfoData {
        os: Some(format!("{:?}", system.os)),
        cpu_info: system.cpu_info().map(|c| format!("{:?}", c)),
        raw: Some(SystemInfoRaw {
            processor_architecture: Some(system.raw.processor_architecture),
            processor_level: Some(system.raw.processor_level),
            processor_revision: Some(system.raw.processor_revision),
            number_of_processors: Some(system.raw.number_of_processors),
            product_type: Some(system.raw.product_type),
            major_version: Some(system.raw.major_version),
            minor_version: Some(system.raw.minor_version),
            build_number: Some(system.raw.build_number),
            platform_id: Some(system.raw.platform_id),
            csd_version_rva: Some(system.raw.csd_version_rva),
            suite_mask: Some(system.raw.suite_mask),
            reserved2: Some(system.raw.reserved2),
            cpu_info_data: Some(system.raw.cpu.data.to_vec()),
            os_version: Some(format!(
                "{}.{}.{}",
                system.raw.major_version, system.raw.minor_version, system.raw.build_number
            )),
            csd_version: system.csd_version().map(|v| v.to_string()),
        }),
        debug: debug_output(system),
    }
}
