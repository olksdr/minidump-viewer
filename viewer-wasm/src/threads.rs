use crate::common::{SafeU64, debug_output};
use crate::context::{StructuredContext, parse_context_registers};
use minidump::{
    Minidump, MinidumpModuleList, MinidumpSystemInfo, MinidumpThreadList, MinidumpThreadNames,
    Module,
};
use minidump_unwind::{
    CallStack, FrameTrust, SystemInfo, symbols::debuginfo::DebugInfoSymbolProvider, walk_stack,
};
use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub enum StackUnwindingMethod {
    Ok,       // Professional stack unwinding succeeded
    Fallback, // Used basic context unwinding
    Failed,   // Couldn't unwind at all
}

#[derive(Serialize)]
pub struct StackInfo {
    pub start_address: SafeU64,
    pub memory_size: u32,
    pub memory_data: Vec<u8>, // Raw stack memory bytes
}

#[derive(Serialize)]
pub struct StackFrame {
    pub instruction_address: SafeU64,
    pub trust_level: String, // "context", "cfi", "frame_pointer", "scan"
    pub module_name: Option<String>, // From module list, not symbols
}

#[derive(Serialize)]
pub struct ThreadData {
    pub thread_id: u32,
    pub name: Option<String>, // Thread name, None if not available
    pub suspend_count: u32,
    pub priority_class: u32,
    pub priority: u32,
    pub teb: SafeU64, // Thread Environment Block address
    pub stack: Option<StackInfo>,
    pub context: Option<StructuredContext>,
    pub stack_frames: Option<Vec<StackFrame>>, // Stack trace from unwinding
    pub debug: Option<String>,                 // Debug output for this specific thread
    pub stack_unwinding_method: StackUnwindingMethod,
}

// Helper function to parse threads into structured format (async version)
pub async fn parse_threads_data_async<'a>(
    threads: &'a MinidumpThreadList<'a>,
    system: Option<&'a MinidumpSystemInfo>,
    thread_names: Option<&'a MinidumpThreadNames>,
    modules: Option<&'a MinidumpModuleList>,
    dump: &'a Minidump<'_, &[u8]>,
) -> Vec<ThreadData> {
    let mut thread_data = Vec::new();

    // Process each thread with proper async stack unwinding
    for thread in &threads.threads {
        // Get basic stack information from raw thread data
        let stack = if thread.raw.stack.start_of_memory_range != 0 {
            Some(StackInfo {
                start_address: thread.raw.stack.start_of_memory_range.into(),
                memory_size: thread.raw.stack.memory.data_size,
                memory_data: Vec::new(), // Will be empty for now to avoid memory access issues
            })
        } else {
            None
        };

        // Get CPU context if available
        let context = system
            .and_then(|s| thread.context(s, None))
            .map(|c| parse_context_registers(&c));

        // Get thread name if available
        let name = thread_names
            .and_then(|names| names.get_name(thread.raw.thread_id))
            .map(|name| name.into_owned());

        // Use proper async stack unwinding with minidump-unwind
        let (stack_frames, unwinding_method) =
            extract_stack_frames_async(thread, system, modules, dump, threads.threads.len() as u32)
                .await;

        thread_data.push(ThreadData {
            thread_id: thread.raw.thread_id,
            name,
            suspend_count: thread.raw.suspend_count,
            priority_class: thread.raw.priority_class,
            priority: thread.raw.priority,
            teb: thread.raw.teb.into(),
            stack,
            context,
            stack_frames,
            debug: debug_output(thread),
            stack_unwinding_method: unwinding_method,
        });
    }

    // Sort threads by TEB address (Thread Environment Block) to ensure consistent ordering
    // from lowest to highest address
    thread_data.sort_by_key(|thread| thread.teb.raw_value());

    thread_data
}

// Extract stack frames using minidump-unwind's walk_stack function
// Returns (stack_frames, unwinding_method)
async fn extract_stack_frames_async<'a>(
    thread: &'a minidump::MinidumpThread<'a>,
    system: Option<&'a MinidumpSystemInfo>,
    modules: Option<&'a MinidumpModuleList>,
    dump: &'a Minidump<'_, &[u8]>,
    thread_count: u32,
) -> (Option<Vec<StackFrame>>, StackUnwindingMethod) {
    let system_info = match system {
        Some(s) => s,
        None => return (None, StackUnwindingMethod::Failed),
    };
    let modules_list = match modules {
        Some(m) => m,
        None => return (None, StackUnwindingMethod::Failed),
    };
    let memory = match dump.get_memory() {
        Some(mem) => mem,
        None => return (None, StackUnwindingMethod::Failed),
    };

    // Check if the CPU architecture is supported by DebugInfoSymbolProvider
    // Based on the source code, only X86_64 and Arm64 are supported, others panic with unimplemented!()
    use minidump::system_info::Cpu;
    let cpu_supported = matches!(system_info.cpu, Cpu::X86_64 | Cpu::Arm64);

    if !cpu_supported {
        // CPU architecture not supported by DebugInfoSymbolProvider, use fallback
        let fallback_frames = fallback_context_unwinding(thread, system, modules);
        return (fallback_frames, StackUnwindingMethod::Fallback);
    }

    // Create DebugInfoSymbolProvider for supported architectures
    let symbol_provider = DebugInfoSymbolProvider::new(system_info, modules_list).await;

    // Get CPU context for this thread
    let context = match system.and_then(|s| thread.context(s, None)) {
        Some(ctx) => ctx,
        None => {
            let fallback_frames = fallback_context_unwinding(thread, system, modules);
            return (fallback_frames, StackUnwindingMethod::Fallback);
        }
    };

    // Create CallStack to hold unwound frames
    let mut call_stack = CallStack::with_context(context.into_owned());

    // Manually create SystemInfo from MinidumpSystemInfo
    let system_info_for_unwind = SystemInfo {
        os: system_info.os,
        os_version: None,
        os_build: None,
        cpu: system_info.cpu,
        cpu_info: system_info.cpu_info().map(|s| s.into_owned()),
        cpu_microcode_version: None,
        cpu_count: thread_count as usize,
    };

    // Use walk_stack to perform professional stack unwinding
    let thread_idx = thread.raw.thread_id as usize;
    let stack_memory = thread.stack_memory(&memory);

    // Walk the stack with proper async handling
    walk_stack(
        thread_idx,
        |_idx: usize, _frame: &minidump_unwind::StackFrame| {
            // Callback for each frame during unwinding - we get results from call_stack
        },
        &mut call_stack,
        stack_memory,
        modules_list,
        &system_info_for_unwind,
        &symbol_provider,
    )
    .await;

    // Convert minidump-unwind stack frames to our format
    let frames: Vec<StackFrame> = call_stack
        .frames
        .iter()
        .map(|frame| {
            let module_name = frame
                .module
                .as_ref()
                .map(|module| module.code_file().to_string());

            StackFrame {
                instruction_address: frame.instruction.into(),
                trust_level: frame_trust_to_string(&frame.trust),
                module_name,
            }
        })
        .collect();

    if frames.is_empty() {
        // Fallback to basic context unwinding if walk_stack produces no frames
        let fallback_frames = fallback_context_unwinding(thread, system, modules);
        (fallback_frames, StackUnwindingMethod::Fallback)
    } else {
        (Some(frames), StackUnwindingMethod::Ok)
    }
}

// Fallback to basic context unwinding
fn fallback_context_unwinding(
    thread: &minidump::MinidumpThread,
    system: Option<&MinidumpSystemInfo>,
    modules: Option<&MinidumpModuleList>,
) -> Option<Vec<StackFrame>> {
    let context = system.and_then(|s| thread.context(s, None))?;
    let instruction_pointer = context.get_instruction_pointer();
    let module_name = modules.and_then(|mods| find_module_for_address(mods, instruction_pointer));

    Some(vec![StackFrame {
        instruction_address: instruction_pointer.into(),
        trust_level: frame_trust_to_string(&FrameTrust::Context),
        module_name,
    }])
}

// Convert FrameTrust enum to our string representation
fn frame_trust_to_string(trust: &FrameTrust) -> String {
    match trust {
        FrameTrust::Context => "context".to_string(),
        FrameTrust::CallFrameInfo => "cfi".to_string(),
        FrameTrust::FramePointer => "frame_pointer".to_string(),
        FrameTrust::Scan => "scan".to_string(),
        FrameTrust::CfiScan => "cfi_scan".to_string(),
        FrameTrust::PreWalked => "pre_walked".to_string(),
        FrameTrust::None => "none".to_string(),
    }
}

// Find module that contains the given address (used by fallback)
fn find_module_for_address(modules: &MinidumpModuleList, address: u64) -> Option<String> {
    for module in modules.iter() {
        let base_address = module.raw.base_of_image;
        let end_address = base_address + module.raw.size_of_image as u64;

        if address >= base_address && address < end_address {
            return Some(module.name.clone());
        }
    }
    None
}
