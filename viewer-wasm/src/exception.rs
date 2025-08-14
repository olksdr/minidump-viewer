use crate::common::SafeU64;
use crate::context::{StructuredContext, parse_context_registers};
use minidump::{MinidumpException, MinidumpSystemInfo};
use serde::Serialize;

#[derive(Serialize)]
pub struct ExceptionRecord {
    // MINIDUMP_EXCEPTION fields (nested inside MINIDUMP_EXCEPTION_STREAM)
    pub exception_code: u32,
    pub exception_flags: u32,
    pub exception_record: SafeU64,
    pub exception_address: SafeU64,
    pub number_parameters: u32,
    pub exception_information: Vec<SafeU64>, // First number_parameters elements
}

#[derive(Serialize)]
pub struct ExceptionStreamRaw {
    // MINIDUMP_EXCEPTION_STREAM fields
    pub thread_id: u32,
    pub exception_record: ExceptionRecord,
    // Note: thread_context is a MINIDUMP_LOCATION_DESCRIPTOR, but the actual context
    // is accessed via the high-level context() method, not raw field access
}

#[derive(Serialize)]
pub struct ExceptionData {
    pub crash_reason: Option<String>,       // from get_crash_reason()
    pub crash_address: Option<SafeU64>,     // from get_crash_address()
    pub thread_id: u32,                     // from get_crashing_thread_id()
    pub context: Option<StructuredContext>, // structured register data
    pub raw: Option<ExceptionStreamRaw>,    // properly nested raw structure
    pub debug: Option<String>,              // raw debug output
    pub context_debug: Option<String>,      // context debug output
}

pub fn parse_exception_info(
    exception: &MinidumpException,
    system: Option<&MinidumpSystemInfo>,
) -> ExceptionData {
    // Get crash reason and address if we have system info for context
    let (crash_reason, crash_address) = system
        .map(|s| {
            (
                Some(exception.get_crash_reason(s.os, s.cpu).to_string()),
                Some(exception.get_crash_address(s.os, s.cpu).into()),
            )
        })
        .unwrap_or((None, None));

    // Get structured context if we have system info
    let (context, context_debug_fallback) = system
        .and_then(|s| exception.context(s, None))
        .map(|c| (Some(parse_context_registers(&c)), Some(format!("{:#?}", c))))
        .unwrap_or((None, None));

    // Extract exception information array (only valid elements)
    let exception_information = exception.raw.exception_record.exception_information
        [0..exception.raw.exception_record.number_parameters as usize]
        .iter()
        .map(|&v| v.into())
        .collect();

    ExceptionData {
        crash_reason,
        crash_address,
        thread_id: exception.get_crashing_thread_id(),
        context,
        raw: Some(ExceptionStreamRaw {
            thread_id: exception.raw.thread_id, // Use thread_id from MINIDUMP_EXCEPTION_STREAM
            exception_record: ExceptionRecord {
                exception_code: exception.raw.exception_record.exception_code,
                exception_flags: exception.raw.exception_record.exception_flags,
                exception_record: exception.raw.exception_record.exception_record.into(),
                exception_address: exception.raw.exception_record.exception_address.into(),
                number_parameters: exception.raw.exception_record.number_parameters,
                exception_information,
            },
        }),
        debug: Some(format!("{:#?}", exception)),
        context_debug: context_debug_fallback,
    }
}
