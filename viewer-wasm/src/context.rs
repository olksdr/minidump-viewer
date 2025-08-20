use crate::common::SafeU64;
use minidump::{MinidumpContext, MinidumpRawContext};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::OnceLock;

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

        let category = categorize_register(&reg_name);
        let register = RegisterValue {
            name: reg_name.clone(),
            value: value.into(),
            valid,
            category: category.clone(),
        };

        // Push to appropriate category vector
        match category.as_str() {
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

// Optimized register categorization using static lookup table
static REGISTER_CATEGORIES: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn get_register_categories() -> &'static HashMap<&'static str, &'static str> {
    REGISTER_CATEGORIES.get_or_init(|| {
        let mut map = HashMap::new();

        // General purpose registers (AMD64/x86)
        for reg in &[
            "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rsp", "rbp", "r8", "r9", "r10", "r11",
            "r12", "r13", "r14", "r15", "eax", "ebx", "ecx", "edx", "esi", "edi", "esp", "ebp",
        ] {
            map.insert(*reg, "general_purpose");
        }

        // Instruction pointer
        for reg in &["rip", "eip", "pc"] {
            map.insert(*reg, "instruction_pointer");
        }

        // Segment registers
        for reg in &["cs", "ds", "es", "fs", "gs", "ss"] {
            map.insert(*reg, "segment");
        }

        // Flags registers
        for reg in &["eflags", "rflags", "context_flags", "cpsr"] {
            map.insert(*reg, "flags");
        }

        map
    })
}

fn categorize_register(name: &str) -> String {
    let lower_name = name.to_lowercase();
    let categories = get_register_categories();

    // Check static lookup first
    if let Some(&category) = categories.get(lower_name.as_str()) {
        return category.to_string();
    }

    // Handle debug registers (dynamic pattern)
    if lower_name.starts_with("dr") {
        return "debug".to_string();
    }

    "other".to_string()
}
