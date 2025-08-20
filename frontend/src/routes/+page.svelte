<script lang="ts">
	import { onMount } from 'svelte';
	import { base } from '$app/paths';
	import SystemInfo from '../components/SystemInfo.svelte';
	import Exception from '../components/Exception.svelte';
	import ThreadList from '../components/ThreadList.svelte';
	import ModuleList from '../components/ModuleList.svelte';
	import MemoryList from '../components/MemoryList.svelte';

	// WASM-related imports (will be loaded dynamically)
	let parse_minidump: ((data: Uint8Array) => Promise<MinidumpResult>) | null = null;

	// Type definition for raw system info fields
	interface SystemInfoRaw {
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

	// Type definition for structured system info
	interface SystemInfoData {
		os?: string;
		cpu_info?: string;
		raw?: SystemInfoRaw;
		debug?: string;
	}

	// Type definition for exception record fields (nested inside MINIDUMP_EXCEPTION_STREAM)
	interface ExceptionRecord {
		exception_code: number;
		exception_flags: number;
		exception_record: number;
		exception_address: number;
		number_parameters: number;
		exception_information: number[];
	}

	// Type definition for MINIDUMP_EXCEPTION_STREAM structure
	interface ExceptionStreamRaw {
		thread_id: number;
		exception_record: ExceptionRecord;
	}

	// Type definition for CPU register
	interface RegisterValue {
		name: string;
		value: number;
		category: string;
		valid: boolean;
	}

	// Type definition for structured CPU context
	interface StructuredContext {
		general_purpose: RegisterValue[];
		instruction_pointer: RegisterValue[];
		segment: RegisterValue[];
		flags: RegisterValue[];
		debug: RegisterValue[];
		other: RegisterValue[];
		architecture: string;
	}

	// Type definition for structured exception info
	interface ExceptionData {
		crash_reason?: string;
		crash_address?: number;
		thread_id: number;
		context?: StructuredContext;
		raw?: ExceptionStreamRaw;
		debug?: string;
		context_debug?: string;
	}

	// Type definition for stack info
	interface StackInfo {
		start_address: string;
		memory_size: number;
		memory_data: number[];
	}

	// Type definition for stack frame
	interface StackFrame {
		instruction_address: string;
		trust_level: string;
		module_name?: string;
	}

	// Type definition for thread data
	interface ThreadData {
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
	}

	// Type definition for module version information
	interface VersionInfo {
		file_version?: string;
		product_version?: string;
		file_flags?: string[];
		file_type?: string;
		file_os?: string;
	}

	// Type definition for module CodeView debug information
	interface CodeViewInfo {
		format: string;
		identifier?: string;
		age?: number;
		pdb_filename?: string;
	}

	// Type definition for individual module information
	interface ModuleInfo {
		name: string;
		base_of_image: string;
		size_of_image: number;
		checksum: number;
		time_date_stamp: number;
		version_info?: VersionInfo;
		cv_record_info?: CodeViewInfo;
		misc_record_present: boolean;
	}

	// Type definition for structured modules data
	interface ModuleData {
		modules: ModuleInfo[];
		modules_count: number;
		debug?: string;
	}

	// Type definition for memory region
	interface MemoryRegion {
		start_address: string;
		end_address: string;
		size: number;
		size_formatted: string;
		has_data: boolean;
		data_size: number;
		address_range: string;
	}

	// Type definition for memory info range
	interface MemoryInfoRange {
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

	// Type definition for memory range map
	interface MemoryRangeMap {
		ranges: MemoryInfoRange[];
		ranges_count: number;
	}

	// Type definition for structured memory data
	interface MemoryData {
		regions: MemoryRegion[];
		regions_count: number;
		memory_info?: MemoryRangeMap;
		has_memory_info_stream: boolean;
		total_memory_size: number;
		total_memory_size_formatted: string;
		debug?: string;
	}

	// Type definition for the minidump parsing result
	interface MinidumpResult {
		streams_present?: string[];
		modules_count?: number;
		threads_count?: number;
		system_info?: SystemInfoData;
		exception_info?: ExceptionData;
		threads_data?: ThreadData[];
		modules_data?: ModuleData;
		memory_data?: MemoryData;
	}

	// Reactive state for file processing
	let isDragOver = false;
	let parsedResult: MinidumpResult | null = null;
	let errorState: { message: string; details: string } | null = null;

	// Helper function to scroll to a section
	function scrollToSection(sectionId: string) {
		const element = document.getElementById(sectionId);
		if (element) {
			element.scrollIntoView({ behavior: 'smooth' });
		}
	}

	// Helper function to scroll to top of page
	function scrollToTop() {
		window.scrollTo({ top: 0, behavior: 'smooth' });
	}

	// Helper function to generate detailed error descriptions
	function getErrorDescription(errorMessage: string): { message: string; details: string } {
		const error = errorMessage.toLowerCase();

		if (error.includes('headermismatch')) {
			return {
				message: 'Invalid minidump file format',
				details:
					'The file does not appear to be a valid minidump (.dmp/.mdmp) file. Please ensure you are uploading a genuine crash dump file generated by your operating system or compatible debugging tools.'
			};
		} else if (error.includes('network') || error.includes('fetch')) {
			return {
				message: 'Network or loading error',
				details:
					'There was a problem loading the WASM parsing module. Please check your internet connection and try refreshing the page.'
			};
		} else if (error.includes('memory') || error.includes('allocation')) {
			return {
				message: 'Memory allocation error',
				details:
					'The file may be too large or corrupted. Try with a smaller minidump file or ensure the file is not corrupted.'
			};
		} else if (error.includes('corrupted') || error.includes('invalid')) {
			return {
				message: 'Corrupted or invalid file',
				details:
					'The minidump file appears to be corrupted or contains invalid data. Try obtaining a fresh copy of the crash dump.'
			};
		} else if (error.includes('unsupported')) {
			return {
				message: 'Unsupported file format',
				details:
					'This minidump format or version is not currently supported by this parser. The file may be from a very old or very new version of your operating system.'
			};
		} else if (error.includes('permission') || error.includes('access')) {
			return {
				message: 'File access error',
				details:
					'Unable to read the selected file. Please ensure the file is not locked by another application and try again.'
			};
		} else {
			return {
				message: 'Parsing error',
				details: `An unexpected error occurred while parsing the minidump file: ${errorMessage.replace('Parse error: ', '')}`
			};
		}
	}

	// Helper function to map stream names to user-friendly titles and section IDs
	function getStreamInfo(streamName: string) {
		const streamMap: Record<string, { title: string; id: string }> = {
			SystemInfo: { title: 'SYSTEM OVERVIEW', id: 'system-overview-section' },
			Exception: { title: 'EXCEPTION', id: 'exception-section' },
			ThreadList: { title: 'THREADS', id: 'threads-section' },
			ModuleList: { title: 'MODULES', id: 'modules-section' },
			MemoryList: { title: 'MEMORY', id: 'memory-section' }
		};
		return (
			streamMap[streamName] || {
				title: streamName.toUpperCase(),
				id: streamName.toLowerCase() + '-section'
			}
		);
	}

	// Helper function to check if a stream has displayable data
	function hasStreamData(streamName: string, result: MinidumpResult): boolean {
		switch (streamName) {
			case 'SystemInfo':
				return !!result.system_info;
			case 'Exception':
				return !!result.exception_info;
			case 'ThreadList':
				return !!result.threads_data;
			case 'ModuleList':
				return !!result.modules_data;
			case 'MemoryList':
				return !!result.memory_data;
			default:
				return false;
		}
	}

	// File input element reference
	let fileInput: HTMLInputElement;

	// Initialize WASM on component mount
	onMount(async () => {
		try {
			// Import WASM module from static assets with proper base path
			const wasmModule = await import(`${base}/wasm/viewer_wasm.js`);
			await wasmModule.default(); // Initialize WASM
			parse_minidump = wasmModule.parse_minidump;
			console.log('WASM module loaded successfully');
		} catch (error) {
			console.error('Failed to load WASM module:', error);
		}
	});

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		isDragOver = true;
	}

	function handleDragLeave() {
		isDragOver = false;
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		isDragOver = false;
		const files = event.dataTransfer?.files;
		if (files && files.length > 0) {
			processFile(files[0]);
		}
	}

	function handleFileInput(event: Event) {
		const target = event.target as HTMLInputElement;
		if (target.files && target.files.length > 0) {
			processFile(target.files[0]);
		}
	}

	async function processFile(file: File) {
		// Clear any previous error state
		errorState = null;

		if (!parse_minidump) {
			errorState = getErrorDescription('WASM module not loaded yet. Please try again.');
			return;
		}

		try {
			const arrayBuffer = await file.arrayBuffer();
			const uint8Array = new Uint8Array(arrayBuffer);
			const result = await parse_minidump(uint8Array);

			// Store the parsed result for reactive display
			parsedResult = result;
		} catch (error) {
			const errorMessage = 'Parse error: ' + String(error);
			errorState = getErrorDescription(errorMessage);
			parsedResult = null;
		}
	}
</script>

<svelte:head>
	<title>Minidump Viewer (WASM)</title>
</svelte:head>

<main class="retro-container">
	{#if !parsedResult}
		<!-- Landing Page Layout: Large centered logo -->
		<div class="retro-landing">
			<img src="{base}/logo.png" alt="Minidump Viewer Logo" class="retro-logo-large" />
		</div>
	{:else}
		<!-- Compact Header for Results Page -->
		<header class="retro-header-compact">
			<img src="{base}/logo.png" alt="Minidump Viewer Logo" class="retro-logo-compact" />
			<div class="retro-haiku">
				<div>Silent crash captured,</div>
				<div>threads whisper their final frames—</div>
				<div>truth sleeps in the dump.</div>
			</div>
		</header>
	{/if}

	<!-- Drag and Drop Area -->
	<div
		class="retro-dropzone {isDragOver ? 'drag-over' : ''}"
		on:dragover={handleDragOver}
		on:dragleave={handleDragLeave}
		on:drop={handleDrop}
		role="button"
		tabindex="0"
	>
		<div class="retro-dropzone-text">
			DRAG & DROP a <span class="retro-code">.dmp</span> file here, or
			<input
				bind:this={fileInput}
				type="file"
				accept=".dmp,.mdmp"
				on:change={handleFileInput}
				style="display: inline; font-family: inherit; font-size: 11px;"
			/>
		</div>
		<div class="retro-dropzone-subtext">All parsing happens locally in your browser.</div>
	</div>

	<!-- Error Message -->
	{#if errorState}
		<div class="retro-error">
			<div class="retro-error-title">ERROR</div>
			<div class="retro-error-message">{errorState.message}</div>
			<div class="retro-error-details">{errorState.details}</div>
		</div>
	{/if}

	<!-- Results -->
	{#if parsedResult}
		<div>
			<!-- System Overview Section -->
			<div class="retro-section" id="system-overview-section">
				<div class="retro-section-header">
					SYSTEM OVERVIEW
					<span
						class="retro-scroll-top"
						on:click={scrollToTop}
						on:keydown={(e) => e.key === 'Enter' && scrollToTop()}
						role="button"
						tabindex="0"
						title="Scroll to top"
					>
						^
					</span>
				</div>
				<div class="retro-content">
					<div
						style="display: flex; gap: 1rem; align-items: flex-start; width: 100%; max-width: 100%; overflow: hidden; box-sizing: border-box;"
					>
						<!-- Overview Panel (15%) -->
						<div style="flex: 0 0 15%; min-width: 0; max-width: 15%; overflow: hidden;">
							<div>
								Streams:
								{#if parsedResult.streams_present && parsedResult.streams_present.length > 0}
									{@const streamsWithData = parsedResult.streams_present.filter((stream) =>
										hasStreamData(stream, parsedResult)
									)}
									{#if streamsWithData.length > 0}
										{#each streamsWithData as stream, i}
											<span
												class="retro-link"
												on:click={() => scrollToSection(getStreamInfo(stream).id)}
												on:keydown={(e) =>
													e.key === 'Enter' && scrollToSection(getStreamInfo(stream).id)}
												role="button"
												tabindex="0"
											>
												{stream}
											</span>{i < streamsWithData.length - 1 ? ', ' : ''}
										{/each}
										{#if parsedResult.streams_present.length > streamsWithData.length}
											<span class="retro-muted">
												(+ {parsedResult.streams_present.length - streamsWithData.length} other streams)</span
											>
										{/if}
									{:else}
										<span class="retro-muted"
											>{parsedResult.streams_present.join(', ')} (no displayable data)</span
										>
									{/if}
								{:else}
									—
								{/if}
							</div>
							<div>Modules: {parsedResult.modules_count ?? '?'}</div>
							<div>Threads: {parsedResult.threads_count ?? '?'}</div>
							<div>Memory Regions: {parsedResult.memory_data?.regions_count ?? '?'}</div>
						</div>

						<!-- System Panel (takes remaining space) -->
						<div style="flex: 1; min-width: 0; overflow: hidden; white-space: normal;">
							{#if parsedResult.system_info}
								<SystemInfo systemInfo={parsedResult.system_info} />
							{:else}
								<span class="retro-muted">No system information available</span>
							{/if}
						</div>
					</div>
				</div>
			</div>

			<!-- Dynamic Sections Based on Available Streams with Data -->
			{#if parsedResult.streams_present}
				{#each parsedResult.streams_present as stream}
					{#if hasStreamData(stream, parsedResult) && stream !== 'SystemInfo'}
						{@const streamInfo = getStreamInfo(stream)}
						<div class="retro-section" id={streamInfo.id}>
							<div class="retro-section-header">
								{streamInfo.title}
								<span
									class="retro-scroll-top"
									on:click={scrollToTop}
									on:keydown={(e) => e.key === 'Enter' && scrollToTop()}
									role="button"
									tabindex="0"
									title="Scroll to top"
								>
									^
								</span>
							</div>
							<div class="retro-content">
								{#if stream === 'Exception' && parsedResult.exception_info}
									<Exception exceptionInfo={parsedResult.exception_info} />
								{:else if stream === 'ThreadList' && parsedResult.threads_data}
									<ThreadList
										threadsData={parsedResult.threads_data}
										crashingThreadId={parsedResult.exception_info?.thread_id}
									/>
								{:else if stream === 'ModuleList' && parsedResult.modules_data}
									<ModuleList moduleData={parsedResult.modules_data} />
								{:else if stream === 'MemoryList' && parsedResult.memory_data}
									<MemoryList memoryData={parsedResult.memory_data} />
								{/if}
							</div>
						</div>
					{/if}
				{/each}
			{/if}
		</div>
	{/if}
</main>
