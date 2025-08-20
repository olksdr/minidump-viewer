use serde::Serialize;
use wasm_bindgen::prelude::*;

mod common;
mod context;
mod debug;
mod errors;
mod exception;
mod memory;
mod modules;
mod system_info;
mod threads;

use minidump::{
    Minidump, MinidumpException, MinidumpMemoryInfoList, MinidumpModuleList, MinidumpSystemInfo,
    MinidumpThreadList, MinidumpThreadNames,
};

use errors::{Result, ViewerError};
use exception::{ExceptionData, parse_exception_info};
use memory::{MemoryData, parse_memory_data, parse_memory_info_data};
use modules::{ModuleData, get_modules_count, parse_modules_data};
use system_info::{SystemInfoData, parse_system_info};
use threads::{ThreadData, parse_threads_data_async};

#[wasm_bindgen]
pub async fn parse_minidump(bytes: &[u8]) -> std::result::Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    parse_minidump_internal(bytes).await.map_err(Into::into)
}

async fn parse_minidump_internal(bytes: &[u8]) -> Result<JsValue> {
    let dump = Minidump::read(bytes)?;
    let streams = extract_minidump_streams(&dump);
    let overview = build_overview(&streams, &dump).await?;
    Ok(serde_wasm_bindgen::to_value(&overview)?)
}

#[derive(Debug)]
struct MinidumpStreams<'a> {
    system: Option<MinidumpSystemInfo>,
    exception: Option<MinidumpException<'a>>,
    threads: Option<MinidumpThreadList<'a>>,
    thread_names: Option<MinidumpThreadNames>,
    modules: Option<MinidumpModuleList>,
    memory: Option<minidump::UnifiedMemoryList<'a>>,
    memory_info: Option<MinidumpMemoryInfoList<'a>>,
}

fn extract_minidump_streams<'a>(dump: &'a Minidump<'a, &'a [u8]>) -> MinidumpStreams<'a> {
    MinidumpStreams {
        system: dump.get_stream::<MinidumpSystemInfo>().ok(),
        exception: dump.get_stream::<MinidumpException>().ok(),
        threads: dump.get_stream::<MinidumpThreadList>().ok(),
        thread_names: dump.get_stream::<MinidumpThreadNames>().ok(),
        modules: dump.get_stream::<MinidumpModuleList>().ok(),
        memory: dump.get_memory(),
        memory_info: dump.get_stream::<MinidumpMemoryInfoList>().ok(),
    }
}

fn build_streams_present_list(streams: &MinidumpStreams) -> Vec<&'static str> {
    let mut streams_present = Vec::new();
    if streams.system.is_some() {
        streams_present.push("SystemInfo");
    }
    if streams.exception.is_some() {
        streams_present.push("Exception");
    }
    if streams.threads.is_some() {
        streams_present.push("ThreadList");
    }
    if streams.modules.is_some() {
        streams_present.push("ModuleList");
    }
    if streams.memory.is_some() {
        streams_present.push("MemoryList");
    }
    streams_present
}

async fn build_overview(
    streams: &MinidumpStreams<'_>,
    dump: &Minidump<'_, &[u8]>,
) -> Result<Overview> {
    let streams_present = build_streams_present_list(streams);

    // Parse individual components
    let system_info = streams.system.as_ref().map(parse_system_info);
    let exception_info = streams
        .exception
        .as_ref()
        .map(|e| parse_exception_info(e, streams.system.as_ref()));

    let threads_data = if let Some(threads_ref) = streams.threads.as_ref() {
        Some(
            parse_threads_data_async(
                threads_ref,
                streams.system.as_ref(),
                streams.thread_names.as_ref(),
                streams.modules.as_ref(),
                dump,
            )
            .await,
        )
    } else {
        None
    };

    let modules_data = streams.modules.as_ref().map(parse_modules_data);
    let memory_data = build_memory_data(streams);

    Ok(Overview {
        streams_present,
        modules_count: streams.modules.as_ref().map(get_modules_count),
        threads_count: streams.threads.as_ref().map(|t| t.threads.len()),
        system_info,
        exception_info,
        threads_data,
        modules_data,
        memory_data,
    })
}

fn build_memory_data(streams: &MinidumpStreams) -> Option<MemoryData> {
    streams.memory.as_ref().map(|m| {
        let mut memory_data = parse_memory_data(m);

        // Add memory info if available
        if let Some(info) = streams.memory_info.as_ref() {
            memory_data.memory_info = Some(parse_memory_info_data(info));
            memory_data.has_memory_info_stream = true;
        }

        memory_data
    })
}

#[derive(Serialize)]
struct Overview {
    streams_present: Vec<&'static str>,
    modules_count: Option<usize>,
    threads_count: Option<usize>,
    system_info: Option<SystemInfoData>,
    exception_info: Option<ExceptionData>,
    threads_data: Option<Vec<ThreadData>>,
    modules_data: Option<ModuleData>,
    memory_data: Option<MemoryData>,
}

// Optional: prove `symbolic` compiles on Wasm and let users drop a PDB/ELF/Mach-O/Breakpad file
// to show basic metadata. This doesn't run during minidump parsing.
#[wasm_bindgen]
pub fn dif_metadata(bytes: &[u8]) -> std::result::Result<JsValue, JsValue> {
    dif_metadata_internal(bytes).map_err(Into::into)
}

fn dif_metadata_internal(bytes: &[u8]) -> Result<JsValue> {
    let meta = debug::parse_dif_metadata(bytes).map_err(ViewerError::DebugInfo)?;
    Ok(serde_wasm_bindgen::to_value(&meta)?)
}
