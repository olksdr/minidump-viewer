<script lang="ts">
	interface MemoryRegion {
		start_address: string;
		end_address: string;
		size: number;
		size_formatted: string;
		has_data: boolean;
		data_size: number;
		address_range: string;
	}

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

	interface MemoryRangeMap {
		ranges: MemoryInfoRange[];
		ranges_count: number;
	}

	interface MemoryData {
		regions: MemoryRegion[];
		regions_count: number;
		memory_info?: MemoryRangeMap;
		has_memory_info_stream: boolean;
		total_memory_size: number;
		total_memory_size_formatted: string;
		debug?: string;
	}

	export let memoryData: MemoryData;

	// State for collapsible sections
	let expandedRegions = new Set<string>();
	let expandedRanges = new Set<string>();
	let expandedSections: Record<string, boolean> = {};

	// Helper to format values
	function formatValue(value: string | number | undefined | null): string {
		if (value === undefined || value === null) return '-';
		return String(value);
	}

	// Helper to format hex values
	function formatHex(value: number): string {
		return `0x${value.toString(16).toLowerCase().padStart(8, '0')}`;
	}

	// Helper to toggle region expansion
	function toggleRegion(regionId: string) {
		if (expandedRegions.has(regionId)) {
			expandedRegions.delete(regionId);
		} else {
			expandedRegions.add(regionId);
		}
		expandedRegions = new Set(expandedRegions);
	}

	// Helper to toggle range expansion
	function toggleRange(rangeId: string) {
		if (expandedRanges.has(rangeId)) {
			expandedRanges.delete(rangeId);
		} else {
			expandedRanges.add(rangeId);
		}
		expandedRanges = new Set(expandedRanges);
	}

	// Helper to toggle nested sections
	function toggleSection(section: string) {
		const oldValue = expandedSections[section] || false;
		expandedSections[section] = !oldValue;
		expandedSections = { ...expandedSections };
	}

	// Helper to generate region identifier
	function getRegionId(region: MemoryRegion, index: number): string {
		return `${region.start_address}-${index}`;
	}

	// Helper to generate range identifier
	function getRangeId(range: MemoryInfoRange, index: number): string {
		return `${range.base_address}-${index}`;
	}
</script>

<div class="font-mono text-retro-sm leading-tight whitespace-normal">
	<!-- Memory Regions Section -->
	<div class="m-0">
		<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
			<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
				>total_memory_size:</span
			>
			<span class="text-retro-text flex-1"
				>{memoryData.total_memory_size_formatted} ({formatValue(memoryData.total_memory_size)} bytes)</span
			>
		</div>
		<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
			<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
				>has_memory_info_stream:</span
			>
			<span class="text-retro-text flex-1">{memoryData.has_memory_info_stream}</span>
		</div>
	</div>

	{#each memoryData.regions as region, index (getRegionId(region, index))}
		{@const regionId = getRegionId(region, index)}
		<div class="m-0 rounded-sm">
			<button
				class="bg-none border-none font-mono text-retro-sm text-retro-text cursor-pointer py-retro-sm px-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-purple"
				on:click={() => toggleRegion(regionId)}
			>
				<span class="text-retro-link mr-retro-md font-bold underline min-w-[12px] text-center"
					>{expandedRegions.has(regionId) ? '-' : '+'}</span
				>
				<span class="font-semibold mr-retro-xl text-retro-purple">Memory Region #{index + 1}</span>
				<span class="text-retro-muted text-retro-xs font-normal ml-retro-xl">
					({region.address_range}, {region.size_formatted})
					{#if region.has_data}<span class="text-retro-purple font-semibold ml-retro-md"
							>[DATA]</span
						>{/if}
				</span>
			</button>

			{#if expandedRegions.has(regionId)}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border border-opacity-15"
				>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>start_address:</span
						>
						<span class="text-retro-text flex-1">{region.start_address}</span>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>end_address:</span
						>
						<span class="text-retro-text flex-1">{region.end_address}</span>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>address_range:</span
						>
						<span class="text-retro-text flex-1">{region.address_range}</span>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>size:</span
						>
						<span class="text-retro-text flex-1"
							>{region.size_formatted} ({formatValue(region.size)} bytes)</span
						>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>has_data:</span
						>
						<span class="text-retro-text flex-1">{region.has_data}</span>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
							>data_size:</span
						>
						<span class="text-retro-text flex-1"
							>{formatValue(region.data_size)} bytes{#if region.data_size !== region.size}
								(of {formatValue(region.size)} total){/if}</span
						>
					</div>
				</div>
			{/if}
		</div>
	{/each}

	<!-- Memory Range Map Section -->
	{#if memoryData.memory_info}
		<div class="mt-retro-4xl mb-retro-xl">
			<div class="text-retro-purple font-semibold text-retro-base mb-retro-md">
				MEMORY RANGE MAP
			</div>
			<div class="m-0">
				<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
					<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
						>ranges_count:</span
					>
					<span class="text-retro-text flex-1">{memoryData.memory_info.ranges_count}</span>
				</div>
			</div>
		</div>

		{#each memoryData.memory_info.ranges as range, index (getRangeId(range, index))}
			{@const rangeId = getRangeId(range, index)}
			<div class="m-0 rounded-sm">
				<button
					class="bg-none border-none font-mono text-retro-sm text-retro-text cursor-pointer py-retro-sm px-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-purple"
					on:click={() => toggleRange(rangeId)}
				>
					<span class="text-retro-link mr-retro-md font-bold underline min-w-[12px] text-center"
						>{expandedRanges.has(rangeId) ? '-' : '+'}</span
					>
					<span class="font-semibold mr-retro-xl text-retro-purple">Range #{index + 1}</span>
					<span class="text-retro-muted text-retro-xs font-normal ml-retro-xl">
						({range.base_address}, {range.region_size_formatted}, {range.protection})
					</span>
				</button>

				{#if expandedRanges.has(rangeId)}
					<div
						class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border border-opacity-15"
					>
						<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
							<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
								>base_address:</span
							>
							<span class="text-retro-text flex-1">{range.base_address}</span>
						</div>
						<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
							<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
								>allocation_base:</span
							>
							<span class="text-retro-text flex-1">{range.allocation_base}</span>
						</div>
						<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
							<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
								>region_size:</span
							>
							<span class="text-retro-text flex-1"
								>{range.region_size_formatted} ({formatValue(range.region_size)} bytes)</span
							>
						</div>
						<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
							<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
								>state:</span
							>
							<span class="text-retro-text flex-1"
								>{range.state} ({formatHex(range.state_value)})</span
							>
						</div>
						<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
							<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
								>protection:</span
							>
							<span class="text-retro-text flex-1"
								>{range.protection} ({formatHex(range.protection_value)})</span
							>
						</div>
						<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
							<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
								>allocation_protection:</span
							>
							<span class="text-retro-text flex-1"
								>{range.allocation_protection} ({formatHex(
									range.allocation_protection_value
								)})</span
							>
						</div>
						<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
							<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
								>memory_type:</span
							>
							<span class="text-retro-text flex-1"
								>{range.memory_type} ({formatHex(range.memory_type_value)})</span
							>
						</div>
					</div>
				{/if}
			</div>
		{/each}
	{:else}
		<div class="mt-retro-4xl mb-retro-xl">
			<div class="text-retro-purple font-semibold text-retro-base mb-retro-md">
				MEMORY RANGE MAP
			</div>
			<div class="m-0">
				<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
					<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']"
						>status:</span
					>
					<span class="text-retro-text flex-1 text-retro-muted">Not available in this minidump</span
					>
				</div>
				<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
					<span class="text-retro-purple min-w-[180px] font-medium after:content-[' ']">note:</span>
					<span class="text-retro-text flex-1 text-retro-muted"
						>Memory protection, state, and type info requires MinidumpMemoryInfoList stream</span
					>
				</div>
			</div>
		</div>
	{/if}

	<!-- Debug Data -->
	{#if memoryData.debug}
		<div class="m-0 rounded-sm">
			<button
				class="bg-none border-none font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-purple"
				on:click={() => toggleSection('debug')}
			>
				<span class="text-retro-link mr-retro-md font-bold underline min-w-[12px] text-center"
					>{expandedSections['debug'] ? '-' : '+'}</span
				>
				<span class="text-retro-text font-medium">debug</span>
			</button>

			{#if expandedSections['debug']}
				<div
					class="m-0 ml-retro-4xl p-retro-md pl-retro-xl bg-gray-50 border-2 border-dashed border-retro-border/30 rounded-retro-sm"
				>
					<pre
						class="text-retro-xs m-0 text-retro-muted whitespace-pre-wrap break-words">{memoryData.debug}</pre>
				</div>
			{/if}
		</div>
	{/if}
</div>
