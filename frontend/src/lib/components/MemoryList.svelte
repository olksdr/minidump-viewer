<script lang="ts">
	import type { MemoryData } from '../types';
	import { formatValue, formatHex, createSetToggleFunction, createToggleFunction } from '../utils';
	import { CollapsibleSection, FieldDisplay, DebugSection } from '..';

	export let memoryData: MemoryData;

	// State for collapsible sections - simplified with better TypeScript
	let expandedRegions = new Set<string>();
	let expandedRanges = new Set<string>();
	let expandedSections: Record<string, boolean> = {};

	// Create toggle functions using utilities
	$: toggleRegion = createSetToggleFunction(
		expandedRegions,
		(newSet) => (expandedRegions = newSet)
	);
	$: toggleRange = createSetToggleFunction(expandedRanges, (newSet) => (expandedRanges = newSet));
	$: toggleSection = createToggleFunction(
		expandedSections,
		(newState) => (expandedSections = newState)
	);

	// Helper to generate region identifier
	function getRegionId(region: { start_address: string }, index: number): string {
		return `${region.start_address}-${index}`;
	}

	// Helper to generate range identifier
	function getRangeId(range: { base_address: string }, index: number): string {
		return `${range.base_address}-${index}`;
	}
</script>

<div class="font-mono text-retro-sm leading-tight whitespace-normal">
	<!-- Memory Regions Section -->
	<FieldDisplay
		label="total_memory_size"
		value="{memoryData.total_memory_size_formatted} ({formatValue(
			memoryData.total_memory_size
		)} bytes)"
	/>
	<FieldDisplay label="has_memory_info_stream" value={String(memoryData.has_memory_info_stream)} />

	{#each memoryData.regions as region, index (getRegionId(region, index))}
		{@const regionId = getRegionId(region, index)}
		{@const subtitle = `(${region.address_range}, ${region.size_formatted})${region.has_data ? ' [DATA]' : ''}`}
		<CollapsibleSection
			expanded={expandedRegions.has(regionId)}
			title="Memory Region #{index + 1}"
			{subtitle}
			titleClass="retro-section-title-with-icon"
			on:toggle={() => toggleRegion(regionId)}
		>
			<FieldDisplay label="start_address" value={region.start_address} />
			<FieldDisplay label="end_address" value={region.end_address} />
			<FieldDisplay label="address_range" value={region.address_range} />
			<FieldDisplay
				label="size"
				value="{region.size_formatted} ({formatValue(region.size)} bytes)"
			/>
			<FieldDisplay label="has_data" value={String(region.has_data)} />
			<FieldDisplay label="data_size" value="" hasSlotContent>
				{formatValue(region.data_size)} bytes{#if region.data_size !== region.size}
					(of {formatValue(region.size)} total){/if}
			</FieldDisplay>
		</CollapsibleSection>
	{/each}

	<!-- Memory Range Map Section -->
	<div class="mt-retro-4xl mb-retro-xl">
		<div class="text-retro-purple font-semibold text-retro-base mb-retro-md">MEMORY RANGE MAP</div>
		{#if memoryData.memory_info}
			<FieldDisplay label="ranges_count" value={memoryData.memory_info.ranges_count} />
		{:else}
			<FieldDisplay
				label="status"
				value="Not available in this minidump"
				valueClass="text-retro-muted"
			/>
			<FieldDisplay
				label="note"
				value="Memory protection, state, and type info requires MinidumpMemoryInfoList stream"
				valueClass="text-retro-muted"
			/>
		{/if}
	</div>

	{#if memoryData.memory_info}
		{#each memoryData.memory_info.ranges as range, index (getRangeId(range, index))}
			{@const rangeId = getRangeId(range, index)}
			{@const rangeSubtitle = `(${range.base_address}, ${range.region_size_formatted}, ${range.protection})`}
			<CollapsibleSection
				expanded={expandedRanges.has(rangeId)}
				title="Range #{index + 1}"
				subtitle={rangeSubtitle}
				titleClass="retro-section-title-with-icon"
				on:toggle={() => toggleRange(rangeId)}
			>
				<FieldDisplay label="base_address" value={range.base_address} />
				<FieldDisplay label="allocation_base" value={range.allocation_base} />
				<FieldDisplay
					label="region_size"
					value="{range.region_size_formatted} ({formatValue(range.region_size)} bytes)"
				/>
				<FieldDisplay label="state" value="{range.state} ({formatHex(range.state_value)})" />
				<FieldDisplay
					label="protection"
					value="{range.protection} ({formatHex(range.protection_value)})"
				/>
				<FieldDisplay
					label="allocation_protection"
					value="{range.allocation_protection} ({formatHex(range.allocation_protection_value)})"
				/>
				<FieldDisplay
					label="memory_type"
					value="{range.memory_type} ({formatHex(range.memory_type_value)})"
				/>
			</CollapsibleSection>
		{/each}
	{/if}

	<!-- Debug Data -->
	{#if memoryData.debug}
		<DebugSection
			debugContent={memoryData.debug}
			expanded={expandedSections['debug'] || false}
			on:toggle={() => toggleSection('debug')}
		/>
	{/if}
</div>
