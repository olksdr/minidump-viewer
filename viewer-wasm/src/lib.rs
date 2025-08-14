use serde::Serialize;
use wasm_bindgen::prelude::*;

mod common;
mod context;
mod debug;
mod exception;
mod memory;
mod modules;
mod system_info;
mod threads;

use minidump::{
    Minidump, MinidumpException, MinidumpMemoryInfoList, MinidumpModuleList, MinidumpSystemInfo,
    MinidumpThreadList, MinidumpThreadNames,
};

use exception::{ExceptionData, parse_exception_info};
use memory::{MemoryData, parse_memory_data, parse_memory_info_data};
use modules::{ModuleData, get_modules_count, parse_modules_data};
use system_info::{SystemInfoData, parse_system_info};
use threads::{ThreadData, parse_threads_data_async};

#[wasm_bindgen]
pub async fn parse_minidump(bytes: &[u8]) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let dump = Minidump::read(bytes)
        .map_err(|e| JsValue::from_str(&format!("minidump read error: {e:?}")))?;

    let system = dump.get_stream::<MinidumpSystemInfo>().ok();
    let exception = dump.get_stream::<MinidumpException>().ok();
    let threads = dump.get_stream::<MinidumpThreadList>().ok();
    let thread_names = dump.get_stream::<MinidumpThreadNames>().ok();
    let modules = dump.get_stream::<MinidumpModuleList>().ok();
    let mem = dump.get_memory();
    let mem_info = dump.get_stream::<MinidumpMemoryInfoList>().ok();

    let mut streams_present = Vec::<&'static str>::new();
    if system.is_some() {
        streams_present.push("SystemInfo");
    }
    if exception.is_some() {
        streams_present.push("Exception");
    }
    if threads.is_some() {
        streams_present.push("ThreadList");
    }
    if modules.is_some() {
        streams_present.push("ModuleList");
    }
    if mem.is_some() {
        streams_present.push("MemoryList");
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

    let system_info = system.as_ref().map(parse_system_info);
    let exception_info = exception
        .as_ref()
        .map(|e| parse_exception_info(e, system.as_ref()));
    let threads_data = if let Some(threads_ref) = threads.as_ref() {
        Some(
            parse_threads_data_async(
                threads_ref,
                system.as_ref(),
                thread_names.as_ref(),
                modules.as_ref(),
                &dump,
            )
            .await,
        )
    } else {
        None
    };
    let modules_data = modules.as_ref().map(parse_modules_data);

    // Parse memory data and add memory info if available
    let memory_data = mem.as_ref().map(|m| {
        let mut memory_data = parse_memory_data(m);

        // If we have memory info list, parse it and add to the memory data
        if let Some(info) = mem_info.as_ref() {
            memory_data.memory_info = Some(parse_memory_info_data(info));
            memory_data.has_memory_info_stream = true;
        }

        memory_data
    });

    let overview = Overview {
        streams_present,
        modules_count: modules.as_ref().map(get_modules_count),
        threads_count: threads.as_ref().map(|t| t.threads.len()),
        system_info,
        exception_info,
        threads_data,
        modules_data,
        memory_data,
    };

    serde_wasm_bindgen::to_value(&overview).map_err(|e| JsValue::from_str(&e.to_string()))
}

// Optional: prove `symbolic` compiles on Wasm and let users drop a PDB/ELF/Mach-O/Breakpad file
// to show basic metadata. This doesn't run during minidump parsing.
#[wasm_bindgen]
pub fn dif_metadata(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let meta = debug::parse_dif_metadata(bytes).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&meta).map_err(|e| JsValue::from_str(&e.to_string()))
}
