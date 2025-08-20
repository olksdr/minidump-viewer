// Shared utility functions for the minidump viewer
// This file consolidates common functionality to eliminate duplication

import type { ArrayFormatResult, HexDumpResult, TrustLevel, TrustLevelStyle } from './types';

// === Value Formatting Utilities ===

/**
 * Formats a value for display, converting undefined/null to dash
 */
export function formatValue(value: string | number | undefined | null): string {
	if (value === undefined || value === null) return '-';
	return String(value);
}

/**
 * Formats a number as a hex address with proper padding
 */
export function formatAddress(address: number | undefined, padding: number = 8): string {
	if (address === undefined) return '-';
	return `0x${address.toString(16).toLowerCase().padStart(padding, '0')}`;
}

/**
 * Formats a hex value with 0x prefix
 */
export function formatHex(value: number, padding: number = 8): string {
	return `0x${value.toString(16).toLowerCase().padStart(padding, '0')}`;
}

/**
 * Formats a register value (already formatted as hex string from backend)
 */
export function formatRegisterValue(value: string): string {
	return value;
}

// === Size Formatting Utilities ===

/**
 * Formats bytes in human readable format (KB, MB, GB)
 */
export function formatSize(bytes: number): string {
	if (bytes === 0) return '0 bytes';
	const k = 1024;
	const sizes = ['bytes', 'KB', 'MB', 'GB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

/**
 * Formats memory size with appropriate units
 */
export function formatMemorySize(size: number): string {
	if (size < 1024) return `${size} bytes`;
	if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
	return `${(size / (1024 * 1024)).toFixed(1)} MB`;
}

// === Array Formatting Utilities ===

/**
 * Formats an array with truncation and expansion controls
 */
export function formatArrayValue(
	value: (number | string)[] | undefined,
	expanded: boolean,
	maxItems: number = 8,
	previewItems: number = 6,
	formatter: (v: number | string) => string = (v) => String(v)
): ArrayFormatResult {
	if (!value || value.length === 0) {
		return { text: '-', hasMore: false, fullLength: 0 };
	}

	if (value.length <= maxItems || expanded) {
		const formattedValues = value.map(formatter);
		return {
			text: `[${formattedValues.join(', ')}]`,
			hasMore: false,
			fullLength: value.length
		};
	}

	const formattedValues = value.slice(0, previewItems).map(formatter);
	return {
		text: `[${formattedValues.join(', ')}, ... +${value.length - previewItems}`,
		hasMore: true,
		fullLength: value.length
	};
}

/**
 * Formats hex array values specifically
 */
export function formatHexArrayValue(
	value: (number | string)[] | undefined,
	expanded: boolean,
	maxItems: number = 8,
	previewItems: number = 6
): ArrayFormatResult {
	return formatArrayValue(value, expanded, maxItems, previewItems, (v) =>
		typeof v === 'string' ? v : `0x${v.toString(16).toLowerCase()}`
	);
}

// === Timestamp Utilities ===

/**
 * Formats a Unix timestamp to readable date string
 */
export function formatTimestamp(timestamp: number): string {
	if (timestamp === 0) return '-';
	try {
		const date = new Date(timestamp * 1000);
		return date.toISOString().replace('T', ' ').substring(0, 19) + ' UTC';
	} catch {
		return `0x${timestamp.toString(16).toLowerCase()}`;
	}
}

/**
 * Formats a checksum value
 */
export function formatChecksum(checksum: number): string {
	if (checksum === 0) return '-';
	return `0x${checksum.toString(16).toLowerCase().padStart(8, '0')}`;
}

// === System Information Utilities ===

/**
 * Gets human-readable processor architecture name
 */
export function getProcessorArchitectureName(arch?: number): string {
	if (arch === undefined) return '-';
	const architectures: Record<number, string> = {
		0: 'PROCESSOR_ARCHITECTURE_INTEL',
		5: 'PROCESSOR_ARCHITECTURE_ARM',
		6: 'PROCESSOR_ARCHITECTURE_IA64',
		9: 'PROCESSOR_ARCHITECTURE_AMD64',
		12: 'PROCESSOR_ARCHITECTURE_ARM64'
	};
	return architectures[arch] || `Unknown (${arch})`;
}

/**
 * Formats OS version from raw system info
 */
export function getOSVersion(raw?: {
	major_version?: number;
	minor_version?: number;
	build_number?: number;
}): string {
	if (!raw?.major_version) return '-';
	let version = `${raw.major_version}`;
	if (raw.minor_version !== undefined) version += `.${raw.minor_version}`;
	if (raw.build_number !== undefined) version += `.${raw.build_number}`;
	return version;
}

// === Thread Utilities ===

/**
 * Gets human-readable priority class name
 */
export function getPriorityClassName(priorityClass: number): string {
	const priorities: Record<number, string> = {
		0x00000020: 'NORMAL_PRIORITY_CLASS',
		0x00000040: 'IDLE_PRIORITY_CLASS',
		0x00000080: 'HIGH_PRIORITY_CLASS',
		0x00000100: 'REALTIME_PRIORITY_CLASS',
		0x00004000: 'BELOW_NORMAL_PRIORITY_CLASS',
		0x00008000: 'ABOVE_NORMAL_PRIORITY_CLASS'
	};
	return priorities[priorityClass] || `Unknown (0x${priorityClass.toString(16)})`;
}

// === Trust Level Utilities ===

/**
 * Gets CSS class for trust level styling
 */
export function getTrustLevelStyle(trustLevel: TrustLevel): TrustLevelStyle {
	switch (trustLevel.toLowerCase()) {
		case 'context':
			return 'trust-high';
		case 'cfi':
			return 'trust-medium-high';
		case 'frame_pointer':
			return 'trust-medium';
		case 'scan':
			return 'trust-low';
		default:
			return 'trust-unknown';
	}
}

/**
 * Formats trust level name for display
 */
export function formatTrustLevel(trustLevel: TrustLevel): string {
	switch (trustLevel.toLowerCase()) {
		case 'context':
			return 'Context';
		case 'cfi':
			return 'CFI';
		case 'frame_pointer':
			return 'Frame Pointer';
		case 'scan':
			return 'Scan';
		default:
			return trustLevel;
	}
}

// === Exception Utilities ===

/**
 * Gets human-readable exception code name
 */
export function getExceptionCodeName(code?: number): string {
	if (code === undefined) return '-';
	const exceptions: Record<number, string> = {
		0xc0000005: 'EXCEPTION_ACCESS_VIOLATION',
		0xc000001d: 'EXCEPTION_ILLEGAL_INSTRUCTION',
		0xc0000094: 'EXCEPTION_INTEGER_DIVIDE_BY_ZERO',
		0xc0000095: 'EXCEPTION_INTEGER_OVERFLOW',
		0xc0000096: 'EXCEPTION_PRIVILEGED_INSTRUCTION',
		0xc00000fd: 'EXCEPTION_STACK_OVERFLOW',
		0xc000008c: 'EXCEPTION_ARRAY_BOUNDS_EXCEEDED',
		0xc000008d: 'EXCEPTION_FLT_DENORMAL_OPERAND',
		0xc000008e: 'EXCEPTION_FLT_DIVIDE_BY_ZERO',
		0xc000008f: 'EXCEPTION_FLT_INEXACT_RESULT',
		0xc0000090: 'EXCEPTION_FLT_INVALID_OPERATION',
		0xc0000091: 'EXCEPTION_FLT_OVERFLOW',
		0xc0000092: 'EXCEPTION_FLT_STACK_CHECK',
		0xc0000093: 'EXCEPTION_FLT_UNDERFLOW',
		0x80000003: 'EXCEPTION_BREAKPOINT',
		0x80000004: 'EXCEPTION_SINGLE_STEP'
	};
	const name = exceptions[code];
	return name ? `${name} (${formatAddress(code)})` : formatAddress(code);
}

/**
 * Gets human-readable exception flags name
 */
export function getExceptionFlagsName(flags?: number): string {
	if (flags === undefined) return '-';
	if (flags === 1) return 'EXCEPTION_NONCONTINUABLE (1)';
	if (flags === 0) return 'CONTINUABLE (0)';
	return `${flags}`;
}

// === File Path Utilities ===

/**
 * Gets filename from full path
 */
export function getModuleFilename(fullPath: string): string {
	const parts = fullPath.split(/[\\/]/);
	return parts[parts.length - 1] || fullPath;
}

// === Hex Dump Utilities ===

/**
 * Generates hex dump preview from byte array
 */
export function generateHexDump(data: number[], limit: number = 64): HexDumpResult {
	const bytes = data.slice(0, limit);
	const lines = [];
	const totalBytes = data.length;

	for (let i = 0; i < bytes.length; i += 16) {
		const chunk = bytes.slice(i, i + 16);
		const offset = i.toString(16).toLowerCase().padStart(4, '0');
		const hex = chunk.map((b) => b.toString(16).toLowerCase().padStart(2, '0')).join(' ');
		const ascii = chunk.map((b) => (b >= 32 && b < 127 ? String.fromCharCode(b) : '.')).join('');
		lines.push(`${offset}  ${hex.padEnd(47)} |${ascii}|`);
	}

	return {
		lines,
		hasMore: data.length > limit,
		totalBytes
	};
}

// === Register Context Utilities ===

/**
 * Gets count of valid vs total registers
 */
export function getValidCount(registers: { valid: boolean }[]): { valid: number; total: number } {
	const valid = registers.filter((r) => r.valid).length;
	return { valid, total: registers.length };
}

/**
 * Gets CSS class for register validity styling
 */
export function getRegisterClass(valid: boolean): string {
	return valid ? '' : 'opacity-60';
}

// === State Management Utilities ===

/**
 * Creates a toggle function for collapsible sections
 */
export function createToggleFunction(
	state: Record<string, boolean>,
	setState: (newState: Record<string, boolean>) => void
): (key: string) => void {
	return (key: string) => {
		setState({ ...state, [key]: !state[key] });
	};
}

/**
 * Creates a toggle function for Set-based state
 */
export function createSetToggleFunction<T>(
	state: Set<T>,
	setState: (newState: Set<T>) => void
): (item: T) => void {
	return (item: T) => {
		const newState = new Set(state);
		if (newState.has(item)) {
			newState.delete(item);
		} else {
			newState.add(item);
		}
		setState(newState);
	};
}
