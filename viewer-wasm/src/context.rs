use crate::common::SafeU64;
use minidump::{MinidumpContext, MinidumpRawContext};
use serde::Serialize;

#[derive(Serialize)]
pub struct RegisterValue {
    pub name: String,
    pub value: SafeU64,
    pub category: String,
    pub valid: bool,
}

#[derive(Serialize)]
pub struct StructuredContext {
    pub general_purpose: Vec<RegisterValue>,
    pub instruction_pointer: Vec<RegisterValue>,
    pub segment: Vec<RegisterValue>,
    pub flags: Vec<RegisterValue>,
    pub debug: Vec<RegisterValue>,
    pub other: Vec<RegisterValue>,
    pub architecture: String,
}

// Helper function to parse context into structured register data
pub fn parse_context_registers(context: &MinidumpContext) -> StructuredContext {
    let mut general_purpose = Vec::new();
    let mut instruction_pointer = Vec::new();
    let mut segment = Vec::new();
    let mut flags = Vec::new();
    let mut debug = Vec::new();
    let mut other = Vec::new();

    // Get architecture from the raw context type
    let architecture = match &context.raw {
        MinidumpRawContext::Amd64(_) => "Amd64",
        MinidumpRawContext::X86(_) => "X86",
        MinidumpRawContext::Arm64(_) => "Arm64",
        MinidumpRawContext::Arm(_) => "Arm",
        _ => "Unknown",
    }
    .to_string();

    // Get valid register names set for checking validity
    let valid_reg_names: std::collections::HashSet<&str> =
        context.valid_registers().map(|(name, _)| name).collect();

    // Get all registers from the context (direct call, no cpu_context())
    for (name, value) in context.registers() {
        let reg_name = name.to_string();
        let valid = valid_reg_names.contains(name);

        let register = RegisterValue {
            name: reg_name.clone(),
            value: value.into(),
            valid,
            category: categorize_register(&reg_name),
        };

        // Categorize registers based on name patterns
        match categorize_register(&reg_name).as_str() {
            "general_purpose" => general_purpose.push(register),
            "instruction_pointer" => instruction_pointer.push(register),
            "segment" => segment.push(register),
            "flags" => flags.push(register),
            "debug" => debug.push(register),
            _ => other.push(register),
        }
    }

    StructuredContext {
        general_purpose,
        instruction_pointer,
        segment,
        flags,
        debug,
        other,
        architecture,
    }
}

// Helper function to categorize registers by name
fn categorize_register(name: &str) -> String {
    match name.to_lowercase().as_str() {
        // General purpose registers (AMD64/x86)
        "rax" | "rbx" | "rcx" | "rdx" | "rsi" | "rdi" | "rsp" | "rbp" | "r8" | "r9" | "r10"
        | "r11" | "r12" | "r13" | "r14" | "r15" | "eax" | "ebx" | "ecx" | "edx" | "esi" | "edi"
        | "esp" | "ebp" => "general_purpose".to_string(),

        // Instruction pointer
        "rip" | "eip" | "pc" => "instruction_pointer".to_string(),

        // Segment registers
        "cs" | "ds" | "es" | "fs" | "gs" | "ss" => "segment".to_string(),

        // Flags registers
        "eflags" | "rflags" | "context_flags" | "cpsr" => "flags".to_string(),

        // Debug registers
        name if name.starts_with("dr") => "debug".to_string(),

        _ => "other".to_string(),
    }
}
