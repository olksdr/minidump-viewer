<script lang="ts">
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

	interface SystemInfoData {
		os?: string;
		cpu_info?: string;
		raw?: SystemInfoRaw;
		debug?: string;
	}

	export let systemInfo: SystemInfoData;

	// State for collapsible sections
	let rawExpanded = false;
	let debugExpanded = false;
	let cpuDataExpanded = false;

	// Helper to format values
	function formatValue(value: string | number | undefined | null): string {
		if (value === undefined || value === null) return '-';
		return String(value);
	}

	// Helper to format arrays (like CPU data) - now returns HTML with interactive links
	function formatArrayValue(
		value: number[] | undefined,
		expanded: boolean
	): { text: string; hasMore: boolean; fullLength: number } {
		if (!value || value.length === 0) return { text: '-', hasMore: false, fullLength: 0 };

		if (value.length <= 10 || expanded) {
			return {
				text: `[${value.join(', ')}]`,
				hasMore: false,
				fullLength: value.length
			};
		}

		return {
			text: `[${value.slice(0, 8).join(', ')}, ... +${value.length - 8}`,
			hasMore: true,
			fullLength: value.length
		};
	}

	// Process architecture decoder
	function getProcessorArchitectureName(arch?: number): string {
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

	// OS Version formatter
	function getOSVersion(raw?: SystemInfoRaw): string {
		if (!raw?.major_version) return '-';
		let version = `${raw.major_version}`;
		if (raw.minor_version !== undefined) version += `.${raw.minor_version}`;
		if (raw.build_number !== undefined) version += `.${raw.build_number}`;
		return version;
	}
</script>

<div class="system-info">
	<div class="system-field">
		<span class="field-name">os:</span>
		<span class="field-value">{formatValue(systemInfo.os)}</span>
	</div>

	<div class="system-field">
		<span class="field-name">version:</span>
		<span class="field-value">{getOSVersion(systemInfo.raw)}</span>
	</div>

	<div class="system-field">
		<span class="field-name">processor_architecture:</span>
		<span class="field-value"
			>{getProcessorArchitectureName(systemInfo.raw?.processor_architecture)}</span
		>
	</div>

	<div class="system-field">
		<span class="field-name">number_of_processors:</span>
		<span class="field-value">{formatValue(systemInfo.raw?.number_of_processors)}</span>
	</div>

	{#if systemInfo.cpu_info}
		<div class="system-field">
			<span class="field-name">cpu_info:</span>
			<span class="field-value">{systemInfo.cpu_info}</span>
		</div>
	{/if}

	<!-- Collapsible Raw Data -->
	{#if systemInfo.raw}
		<div class="collapsible-section">
			<button class="toggle-button" on:click={() => (rawExpanded = !rawExpanded)}>
				<span class="toggle-icon">{rawExpanded ? '-' : '+'}</span>
				<span class="section-title">raw</span>
			</button>

			{#if rawExpanded}
				<div class="expanded-content">
					<div class="system-field">
						<span class="field-name">processor_level:</span>
						<span class="field-value">{formatValue(systemInfo.raw.processor_level)}</span>
					</div>
					<div class="system-field">
						<span class="field-name">processor_revision:</span>
						<span class="field-value">{formatValue(systemInfo.raw.processor_revision)}</span>
					</div>
					<div class="system-field">
						<span class="field-name">product_type:</span>
						<span class="field-value">{formatValue(systemInfo.raw.product_type)}</span>
					</div>
					<div class="system-field">
						<span class="field-name">platform_id:</span>
						<span class="field-value">{formatValue(systemInfo.raw.platform_id)}</span>
					</div>
					<div class="system-field">
						<span class="field-name">suite_mask:</span>
						<span class="field-value">{formatValue(systemInfo.raw.suite_mask)}</span>
					</div>
					<div class="system-field">
						<span class="field-name">csd_version_rva:</span>
						<span class="field-value">{formatValue(systemInfo.raw.csd_version_rva)}</span>
					</div>
					<div class="system-field">
						<span class="field-name">reserved2:</span>
						<span class="field-value">{formatValue(systemInfo.raw.reserved2)}</span>
					</div>
					{#if systemInfo.raw.os_version}
						<div class="system-field">
							<span class="field-name">os_version:</span>
							<span class="field-value">{systemInfo.raw.os_version}</span>
						</div>
					{/if}
					{#if systemInfo.raw.csd_version}
						<div class="system-field">
							<span class="field-name">csd_version:</span>
							<span class="field-value">{systemInfo.raw.csd_version}</span>
						</div>
					{/if}
					{#if systemInfo.raw.cpu_info_data}
						{@const arrayData = formatArrayValue(systemInfo.raw.cpu_info_data, cpuDataExpanded)}
						<div class="system-field">
							<span class="field-name">cpu_info_data:</span>
							<span class="field-value">
								{arrayData.text}
								{#if arrayData.hasMore}
									<span
										class="array-toggle"
										on:click={() => (cpuDataExpanded = true)}
										on:keydown={(e) => e.key === 'Enter' && (cpuDataExpanded = true)}
										role="button"
										tabindex="0"
									>
										more]
									</span>
								{:else if cpuDataExpanded && arrayData.fullLength > 10}
									<span
										class="array-toggle"
										on:click={() => (cpuDataExpanded = false)}
										on:keydown={(e) => e.key === 'Enter' && (cpuDataExpanded = false)}
										role="button"
										tabindex="0"
									>
										less
									</span>
								{/if}
							</span>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Collapsible Debug Data -->
	{#if systemInfo.debug}
		<div class="collapsible-section">
			<button class="toggle-button" on:click={() => (debugExpanded = !debugExpanded)}>
				<span class="toggle-icon">{debugExpanded ? '-' : '+'}</span>
				<span class="section-title">debug</span>
			</button>

			{#if debugExpanded}
				<div class="expanded-content raw-content">
					<pre>{systemInfo.debug}</pre>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.system-info {
		font-family: 'JetBrains Mono', monospace;
		font-size: 11px;
		line-height: 1.2;
		width: 100%;
		max-width: 100%;
		overflow: hidden;
		box-sizing: border-box;
		white-space: normal;
	}

	.system-field {
		display: flex;
		margin: 0;
		padding: 0;
		padding-left: 16px;
		max-width: 100%;
		overflow: hidden;
		box-sizing: border-box;
		line-height: 1.2;
	}

	.field-name {
		color: var(--retro-accent);
		min-width: 180px;
		max-width: 180px;
		font-weight: 500;
		flex-shrink: 0;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.field-name::after {
		content: ' ';
	}

	.field-value {
		color: var(--retro-text);
		flex: 1;
		min-width: 0;
		overflow: hidden;
		word-wrap: break-word;
		word-break: break-all;
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

	.toggle-icon {
		color: var(--retro-link);
		margin-right: 4px;
		font-weight: 700;
		text-decoration: underline;
		min-width: 12px;
		text-align: center;
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
		overflow: hidden;
		word-wrap: break-word;
		max-width: calc(100% - 24px);
		box-sizing: border-box;
	}

	.raw-content {
		background: rgba(0, 0, 0, 0.02);
		max-width: 100%;
		overflow: hidden;
	}

	.raw-content pre {
		font-size: 10px;
		margin: 0;
		color: var(--retro-muted);
		white-space: pre-wrap;
		word-break: break-all;
		overflow-wrap: break-word;
		max-width: 100%;
		overflow: hidden;
		box-sizing: border-box;
	}

	.array-toggle {
		color: var(--retro-link);
		cursor: pointer;
		text-decoration: underline;
		font-size: 11px;
		font-family: 'JetBrains Mono', monospace;
		transition: all 0.2s ease;
		padding: 1px 2px;
		background: transparent;
	}

	.array-toggle:hover {
		color: var(--retro-text);
		background: var(--retro-highlight);
	}
</style>
