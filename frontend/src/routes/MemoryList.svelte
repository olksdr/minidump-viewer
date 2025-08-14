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

<div class="memory-list">
	<!-- Memory Regions Section -->
	<div class="memory-summary">
		<div class="memory-field">
			<span class="field-name">total_memory_size:</span>
			<span class="field-value"
				>{memoryData.total_memory_size_formatted} ({formatValue(memoryData.total_memory_size)} bytes)</span
			>
		</div>
		<div class="memory-field">
			<span class="field-name">has_memory_info_stream:</span>
			<span class="field-value">{memoryData.has_memory_info_stream}</span>
		</div>
	</div>

	{#each memoryData.regions as region, index (getRegionId(region, index))}
		{@const regionId = getRegionId(region, index)}
		<div class="memory-section">
			<button class="memory-toggle" on:click={() => toggleRegion(regionId)}>
				<span class="toggle-icon">{expandedRegions.has(regionId) ? '-' : '+'}</span>
				<span class="memory-title">Memory Region #{index + 1}</span>
				<span class="memory-info">
					({region.address_range}, {region.size_formatted})
					{#if region.has_data}<span class="data-indicator">[DATA]</span>{/if}
				</span>
			</button>

			{#if expandedRegions.has(regionId)}
				<div class="memory-content">
					<div class="memory-field">
						<span class="field-name">start_address:</span>
						<span class="field-value">{region.start_address}</span>
					</div>
					<div class="memory-field">
						<span class="field-name">end_address:</span>
						<span class="field-value">{region.end_address}</span>
					</div>
					<div class="memory-field">
						<span class="field-name">address_range:</span>
						<span class="field-value">{region.address_range}</span>
					</div>
					<div class="memory-field">
						<span class="field-name">size:</span>
						<span class="field-value"
							>{region.size_formatted} ({formatValue(region.size)} bytes)</span
						>
					</div>
					<div class="memory-field">
						<span class="field-name">has_data:</span>
						<span class="field-value">{region.has_data}</span>
					</div>
					<div class="memory-field">
						<span class="field-name">data_size:</span>
						<span class="field-value"
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
		<div class="section-divider">
			<div class="section-header">MEMORY RANGE MAP</div>
			<div class="memory-summary">
				<div class="memory-field">
					<span class="field-name">ranges_count:</span>
					<span class="field-value">{memoryData.memory_info.ranges_count}</span>
				</div>
			</div>
		</div>

		{#each memoryData.memory_info.ranges as range, index (getRangeId(range, index))}
			{@const rangeId = getRangeId(range, index)}
			<div class="memory-section">
				<button class="memory-toggle" on:click={() => toggleRange(rangeId)}>
					<span class="toggle-icon">{expandedRanges.has(rangeId) ? '-' : '+'}</span>
					<span class="memory-title">Range #{index + 1}</span>
					<span class="memory-info">
						({range.base_address}, {range.region_size_formatted}, {range.protection})
					</span>
				</button>

				{#if expandedRanges.has(rangeId)}
					<div class="memory-content">
						<div class="memory-field">
							<span class="field-name">base_address:</span>
							<span class="field-value">{range.base_address}</span>
						</div>
						<div class="memory-field">
							<span class="field-name">allocation_base:</span>
							<span class="field-value">{range.allocation_base}</span>
						</div>
						<div class="memory-field">
							<span class="field-name">region_size:</span>
							<span class="field-value"
								>{range.region_size_formatted} ({formatValue(range.region_size)} bytes)</span
							>
						</div>
						<div class="memory-field">
							<span class="field-name">state:</span>
							<span class="field-value">{range.state} ({formatHex(range.state_value)})</span>
						</div>
						<div class="memory-field">
							<span class="field-name">protection:</span>
							<span class="field-value"
								>{range.protection} ({formatHex(range.protection_value)})</span
							>
						</div>
						<div class="memory-field">
							<span class="field-name">allocation_protection:</span>
							<span class="field-value"
								>{range.allocation_protection} ({formatHex(
									range.allocation_protection_value
								)})</span
							>
						</div>
						<div class="memory-field">
							<span class="field-name">memory_type:</span>
							<span class="field-value"
								>{range.memory_type} ({formatHex(range.memory_type_value)})</span
							>
						</div>
					</div>
				{/if}
			</div>
		{/each}
	{:else}
		<div class="section-divider">
			<div class="section-header">MEMORY RANGE MAP</div>
			<div class="memory-summary">
				<div class="memory-field">
					<span class="field-name">status:</span>
					<span class="field-value retro-muted">Not available in this minidump</span>
				</div>
				<div class="memory-field">
					<span class="field-name">note:</span>
					<span class="field-value retro-muted"
						>Memory protection, state, and type info requires MinidumpMemoryInfoList stream</span
					>
				</div>
			</div>
		</div>
	{/if}

	<!-- Debug Data -->
	{#if memoryData.debug}
		<div class="collapsible-section">
			<button class="toggle-button" on:click={() => toggleSection('debug')}>
				<span class="toggle-icon">{expandedSections['debug'] ? '-' : '+'}</span>
				<span class="section-title">debug</span>
			</button>

			{#if expandedSections['debug']}
				<div class="expanded-content raw-content">
					<pre>{memoryData.debug}</pre>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.memory-list {
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		line-height: 1.2;
		white-space: normal;
	}

	.memory-summary {
		margin: 0;
	}

	.memory-field {
		display: flex;
		margin: 0;
		padding: 0;
		padding-left: 16px;
		line-height: 1.2;
	}

	.field-name {
		color: var(--retro-accent);
		min-width: 180px;
		font-weight: 500;
	}

	.field-name::after {
		content: ' ';
	}

	.field-value {
		color: var(--retro-text);
		flex: 1;
	}

	.section-divider {
		margin-top: 16px;
		margin-bottom: 8px;
	}

	.section-header {
		color: var(--retro-accent);
		font-weight: 600;
		font-size: 12px;
		margin-bottom: 4px;
	}

	.memory-section {
		margin: 0;
		border-radius: 2px;
	}

	.memory-toggle {
		background: none;
		border: none;
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		color: var(--retro-text);
		cursor: pointer;
		padding: 2px 0;
		display: flex;
		align-items: center;
		width: 100%;
		text-align: left;
		transition: all 0.2s ease;
	}

	.memory-toggle:hover {
		color: var(--retro-accent);
	}

	.toggle-icon {
		color: var(--retro-link);
		margin-right: 4px;
		font-weight: 700;
		text-decoration: underline;
		min-width: 12px;
		text-align: center;
	}

	.memory-title {
		font-weight: 600;
		margin-right: 8px;
		color: var(--retro-accent);
	}

	.memory-info {
		color: var(--retro-muted);
		font-size: 10px;
		font-weight: 400;
		margin-left: 8px;
	}

	.data-indicator {
		color: var(--retro-accent);
		font-weight: 600;
		margin-left: 4px;
	}

	.memory-content {
		margin: 0;
		margin-left: 16px;
		padding: 0;
		padding-left: 8px;
		background: transparent;
		border-left: 2px solid rgba(74, 144, 164, 0.15);
	}

	.collapsible-section {
		margin: 0;
		border-radius: 2px;
	}

	.toggle-button {
		background: none;
		border: none;
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		color: var(--retro-text);
		cursor: pointer;
		padding: 0;
		display: flex;
		align-items: center;
		width: 100%;
		text-align: left;
		transition: all 0.2s ease;
	}

	.toggle-button:hover {
		color: var(--retro-accent);
	}

	.section-title {
		color: var(--retro-text);
		font-weight: 500;
	}

	.expanded-content {
		margin: 0;
		margin-left: 16px;
		padding: 0;
		padding-left: 8px;
		background: transparent;
		border-left: 2px solid rgba(74, 144, 164, 0.15);
	}

	.raw-content {
		background: rgba(0, 0, 0, 0.02);
	}

	.raw-content pre {
		font-size: 10px;
		margin: 0;
		color: var(--retro-muted);
		white-space: pre-wrap;
		word-break: break-word;
	}

	.retro-muted {
		color: var(--retro-muted);
	}
</style>
