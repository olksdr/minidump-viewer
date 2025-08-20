// Shared TypeScript interfaces and types for the minidump viewer
// This file consolidates all type definitions to eliminate duplication

// === Core Register and Context Types ===
export interface RegisterValue {
	name: string;
	value: number;
	category: string;
	valid: boolean;
}

export interface StructuredContext {
	general_purpose: RegisterValue[];
	instruction_pointer: RegisterValue[];
	segment: RegisterValue[];
	flags: RegisterValue[];
	debug: RegisterValue[];
	other: RegisterValue[];
	architecture: string;
}

// === System Information Types ===
export interface SystemInfoRaw {
	processor_architecture?: number;
	processor_level?: number;
	processor_revision?: number;
	number_of_processors?: number;
	product_type?: number;
	major_version?: number;
	minor_version?: number;
	build_number?: number;
	platform_id?: number;
	csd_version_rva?: number;
	suite_mask?: number;
	reserved2?: number;
	cpu_info_data?: number[];
	os_version?: string;
	csd_version?: string;
}

export interface SystemInfoData {
	os?: string;
	cpu_info?: string;
	raw?: SystemInfoRaw;
	debug?: string;
}

// === Exception Types ===
export interface ExceptionRecord {
	exception_code: number;
	exception_flags: number;
	exception_record: number;
	exception_address: number;
	number_parameters: number;
	exception_information: number[];
}

export interface ExceptionStreamRaw {
	thread_id: number;
	exception_record: ExceptionRecord;
}

export interface ExceptionData {
	crash_reason?: string;
	crash_address?: number;
	thread_id: number;
	context?: StructuredContext;
	raw?: ExceptionStreamRaw;
	debug?: string;
	context_debug?: string;
}

// === Thread Types ===
export interface StackInfo {
	start_address: string;
	memory_size: number;
	memory_data: number[];
}

export interface StackFrame {
	instruction_address: string;
	trust_level: string;
	module_name?: string;
}

export interface ThreadData {
	thread_id: number;
	name?: string;
	suspend_count: number;
	priority_class: number;
	priority: number;
	teb: string;
	stack?: StackInfo;
	context?: StructuredContext;
	stack_frames?: StackFrame[];
	debug?: string;
	stack_unwinding_method: 'Ok' | 'Fallback' | 'Failed';
}

// === Module Types ===
export interface VersionInfo {
	file_version?: string;
	product_version?: string;
	file_flags?: string[];
	file_type?: string;
	file_os?: string;
}

export interface CodeViewInfo {
	format: string;
	identifier?: string;
	age?: number;
	pdb_filename?: string;
}

export interface ModuleInfo {
	name: string;
	base_of_image: string;
	size_of_image: number;
	checksum: number;
	time_date_stamp: number;
	version_info?: VersionInfo;
	cv_record_info?: CodeViewInfo;
	misc_record_present: boolean;
}

export interface ModuleData {
	modules: ModuleInfo[];
	modules_count: number;
	debug?: string;
}

// === Memory Types ===
export interface MemoryRegion {
	start_address: string;
	end_address: string;
	size: number;
	size_formatted: string;
	has_data: boolean;
	data_size: number;
	address_range: string;
}

export interface MemoryInfoRange {
	base_address: string;
	allocation_base: string;
	region_size: number;
	region_size_formatted: string;
	state: string;
	state_value: number;
	protection: string;
	protection_value: number;
	allocation_protection: string;
	allocation_protection_value: number;
	memory_type: string;
	memory_type_value: number;
}

export interface MemoryRangeMap {
	ranges: MemoryInfoRange[];
	ranges_count: number;
}

export interface MemoryData {
	regions: MemoryRegion[];
	regions_count: number;
	memory_info?: MemoryRangeMap;
	has_memory_info_stream: boolean;
	total_memory_size: number;
	total_memory_size_formatted: string;
	debug?: string;
}

// === Main Result Type ===
export interface MinidumpResult {
	streams_present?: string[];
	modules_count?: number;
	threads_count?: number;
	system_info?: SystemInfoData;
	exception_info?: ExceptionData;
	threads_data?: ThreadData[];
	modules_data?: ModuleData;
	memory_data?: MemoryData;
}

// === Component Helper Types ===
export interface CollapsibleState {
	[key: string]: boolean;
}

export interface ArrayFormatResult {
	text: string;
	hasMore: boolean;
	fullLength: number;
}

export interface HexDumpResult {
	lines: string[];
	hasMore: boolean;
	totalBytes: number;
}

// === Trust Level Types for Stack Frames ===
export type TrustLevel = 'context' | 'cfi' | 'frame_pointer' | 'scan' | string;

export type TrustLevelStyle =
	| 'trust-high'
	| 'trust-medium-high'
	| 'trust-medium'
	| 'trust-low'
	| 'trust-unknown';
