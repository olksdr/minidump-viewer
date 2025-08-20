<script lang="ts">
	import type { ModuleData } from '../types';
	import {
		formatValue,
		formatSize,
		formatTimestamp,
		formatChecksum,
		getModuleFilename,
		createSetToggleFunction,
		createToggleFunction
	} from '../utils';
	import { CollapsibleSection, FieldDisplay, DebugSection } from '..';

	export let moduleData: ModuleData;

	// State for collapsible sections - simplified with better TypeScript
	let expandedModules = new Set<string>();
	let expandedSections: Record<string, boolean> = {};

	// Create toggle functions using utilities
	$: toggleModule = createSetToggleFunction(
		expandedModules,
		(newSet) => (expandedModules = newSet)
	);
	$: toggleSection = createToggleFunction(
		expandedSections,
		(newState) => (expandedSections = newState)
	);

	// Helper to generate module identifier for state tracking
	function getModuleId(module: { base_of_image: string; name: string }, index: number): string {
		// Use base address as primary identifier, fall back to name + index
		return module.base_of_image || `${module.name}-${index}`;
	}

	// Helper to create section keys for nested state
	function getSectionKey(moduleId: string, section: string): string {
		return `${moduleId}-${section}`;
	}
</script>

<div class="font-mono text-retro-sm leading-tight whitespace-normal">
	{#each moduleData.modules as module, index (getModuleId(module, index))}
		{@const moduleId = getModuleId(module, index)}
		{@const isExpanded = expandedModules.has(moduleId)}
		<CollapsibleSection
			expanded={isExpanded}
			title={getModuleFilename(module.name)}
			subtitle="({module.base_of_image}, {formatSize(module.size_of_image)})"
			titleClass="retro-section-title-with-icon"
			on:toggle={() => toggleModule(moduleId)}
		>
			<!-- Basic module info -->
			<FieldDisplay label="name" value={module.name} />
			<FieldDisplay label="base_of_image" value={module.base_of_image} />
			<FieldDisplay
				label="size_of_image"
				value="{formatSize(module.size_of_image)} ({formatValue(module.size_of_image)} bytes)"
			/>
			<FieldDisplay label="checksum" value={module.checksum} formatter={formatChecksum} />
			<FieldDisplay
				label="time_date_stamp"
				value={module.time_date_stamp}
				formatter={formatTimestamp}
			/>
			<FieldDisplay label="misc_record_present" value={String(module.misc_record_present)} />

			<!-- Version Information -->
			{#if module.version_info}
				{@const versionKey = getSectionKey(moduleId, 'version')}
				<CollapsibleSection
					expanded={expandedSections[versionKey] || false}
					title="version_info"
					nested
					on:toggle={() => toggleSection(versionKey)}
				>
					{#if module.version_info.file_version}
						<FieldDisplay label="file_version" value={module.version_info.file_version} />
					{/if}
					{#if module.version_info.product_version}
						<FieldDisplay label="product_version" value={module.version_info.product_version} />
					{/if}
					{#if module.version_info.file_type}
						<FieldDisplay label="file_type" value={module.version_info.file_type} />
					{/if}
					{#if module.version_info.file_os}
						<FieldDisplay label="file_os" value={module.version_info.file_os} />
					{/if}
					{#if module.version_info.file_flags && module.version_info.file_flags.length > 0}
						<FieldDisplay label="file_flags" value={module.version_info.file_flags.join(', ')} />
					{/if}
				</CollapsibleSection>
			{/if}

			<!-- Debug Information -->
			{#if module.cv_record_info}
				{@const debugKey = getSectionKey(moduleId, 'debug')}
				<CollapsibleSection
					expanded={expandedSections[debugKey] || false}
					title="debug_info"
					subtitle="({module.cv_record_info.format})"
					nested
					on:toggle={() => toggleSection(debugKey)}
				>
					<FieldDisplay label="format" value={module.cv_record_info.format} />
					{#if module.cv_record_info.identifier}
						<FieldDisplay label="identifier" value={module.cv_record_info.identifier} />
					{/if}
					{#if module.cv_record_info.age !== undefined && module.cv_record_info.age !== null}
						<FieldDisplay label="age" value={module.cv_record_info.age} />
					{/if}
					{#if module.cv_record_info.pdb_filename}
						<FieldDisplay label="pdb_filename" value={module.cv_record_info.pdb_filename} />
					{/if}
				</CollapsibleSection>
			{/if}
		</CollapsibleSection>
	{/each}

	<!-- Debug Data -->
	{#if moduleData.debug}
		<DebugSection
			debugContent={moduleData.debug}
			expanded={expandedSections['global-debug'] || false}
			on:toggle={() => toggleSection('global-debug')}
		/>
	{/if}
</div>
