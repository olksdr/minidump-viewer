<script lang="ts">
	import type { SystemInfoData } from '../types';
	import { getProcessorArchitectureName, getOSVersion } from '../utils';
	import { CollapsibleSection, FieldDisplay, DebugSection, ExpandableArray } from '..';

	export let systemInfo: SystemInfoData;

	// State for collapsible sections - simplified with better TypeScript
	let rawExpanded = false;
	let cpuDataExpanded = false;
</script>

<div
	class="font-mono text-retro-sm leading-tight w-full max-w-full overflow-hidden box-border whitespace-normal"
>
	<FieldDisplay label="os" value={systemInfo.os} />
	<FieldDisplay label="version" value={getOSVersion(systemInfo.raw)} />
	<FieldDisplay
		label="processor_architecture"
		value={getProcessorArchitectureName(systemInfo.raw?.processor_architecture)}
	/>
	<FieldDisplay label="number_of_processors" value={systemInfo.raw?.number_of_processors} />

	{#if systemInfo.cpu_info}
		<FieldDisplay label="cpu_info" value={systemInfo.cpu_info} />
	{/if}

	<!-- Collapsible Raw Data -->
	{#if systemInfo.raw}
		<CollapsibleSection bind:expanded={rawExpanded} title="raw">
			<FieldDisplay label="processor_level" value={systemInfo.raw.processor_level} />
			<FieldDisplay label="processor_revision" value={systemInfo.raw.processor_revision} />
			<FieldDisplay label="product_type" value={systemInfo.raw.product_type} />
			<FieldDisplay label="platform_id" value={systemInfo.raw.platform_id} />
			<FieldDisplay label="suite_mask" value={systemInfo.raw.suite_mask} />
			<FieldDisplay label="csd_version_rva" value={systemInfo.raw.csd_version_rva} />
			<FieldDisplay label="reserved2" value={systemInfo.raw.reserved2} />

			{#if systemInfo.raw.os_version}
				<FieldDisplay label="os_version" value={systemInfo.raw.os_version} />
			{/if}

			{#if systemInfo.raw.csd_version}
				<FieldDisplay label="csd_version" value={systemInfo.raw.csd_version} />
			{/if}

			{#if systemInfo.raw.cpu_info_data}
				<FieldDisplay label="cpu_info_data" value="" hasSlotContent>
					<ExpandableArray
						values={systemInfo.raw.cpu_info_data}
						bind:expanded={cpuDataExpanded}
						maxItems={10}
						previewItems={8}
					/>
				</FieldDisplay>
			{/if}
		</CollapsibleSection>
	{/if}

	<!-- Debug Data -->
	{#if systemInfo.debug}
		<DebugSection debugContent={systemInfo.debug} />
	{/if}
</div>
