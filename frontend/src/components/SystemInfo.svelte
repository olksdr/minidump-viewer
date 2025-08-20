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

<div
	class="font-mono text-retro-sm leading-tight w-full max-w-full overflow-hidden box-border whitespace-normal"
>
	<div class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight">
		<span
			class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
			>os:</span
		>
		<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
			>{formatValue(systemInfo.os)}</span
		>
	</div>

	<div class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight">
		<span
			class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
			>version:</span
		>
		<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
			>{getOSVersion(systemInfo.raw)}</span
		>
	</div>

	<div class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight">
		<span
			class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
			>processor_architecture:</span
		>
		<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
			>{getProcessorArchitectureName(systemInfo.raw?.processor_architecture)}</span
		>
	</div>

	<div class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight">
		<span
			class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
			>number_of_processors:</span
		>
		<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
			>{formatValue(systemInfo.raw?.number_of_processors)}</span
		>
	</div>

	{#if systemInfo.cpu_info}
		<div class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight">
			<span
				class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
				>cpu_info:</span
			>
			<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
				>{systemInfo.cpu_info}</span
			>
		</div>
	{/if}

	<!-- Collapsible Raw Data -->
	{#if systemInfo.raw}
		<div class="m-0 rounded-sm">
			<button
				class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-purple"
				on:click={() => (rawExpanded = !rawExpanded)}
			>
				<span class="text-retro-link mr-retro-md font-bold underline min-w-[12px] text-center"
					>{rawExpanded ? '-' : '+'}</span
				>
				<span class="text-retro-text font-medium">raw</span>
			</button>

			{#if rawExpanded}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-border/15 overflow-hidden break-words max-w-[calc(100%-24px)] box-border"
				>
					<div
						class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight"
					>
						<span
							class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
							>processor_level:</span
						>
						<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
							>{formatValue(systemInfo.raw.processor_level)}</span
						>
					</div>
					<div
						class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight"
					>
						<span
							class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
							>processor_revision:</span
						>
						<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
							>{formatValue(systemInfo.raw.processor_revision)}</span
						>
					</div>
					<div
						class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight"
					>
						<span
							class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
							>product_type:</span
						>
						<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
							>{formatValue(systemInfo.raw.product_type)}</span
						>
					</div>
					<div
						class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight"
					>
						<span
							class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
							>platform_id:</span
						>
						<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
							>{formatValue(systemInfo.raw.platform_id)}</span
						>
					</div>
					<div
						class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight"
					>
						<span
							class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
							>suite_mask:</span
						>
						<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
							>{formatValue(systemInfo.raw.suite_mask)}</span
						>
					</div>
					<div
						class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight"
					>
						<span
							class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
							>csd_version_rva:</span
						>
						<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
							>{formatValue(systemInfo.raw.csd_version_rva)}</span
						>
					</div>
					<div
						class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight"
					>
						<span
							class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
							>reserved2:</span
						>
						<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
							>{formatValue(systemInfo.raw.reserved2)}</span
						>
					</div>
					{#if systemInfo.raw.os_version}
						<div
							class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight"
						>
							<span
								class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
								>os_version:</span
							>
							<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
								>{systemInfo.raw.os_version}</span
							>
						</div>
					{/if}
					{#if systemInfo.raw.csd_version}
						<div
							class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight"
						>
							<span
								class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
								>csd_version:</span
							>
							<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all"
								>{systemInfo.raw.csd_version}</span
							>
						</div>
					{/if}
					{#if systemInfo.raw.cpu_info_data}
						{@const arrayData = formatArrayValue(systemInfo.raw.cpu_info_data, cpuDataExpanded)}
						<div
							class="flex m-0 p-0 pl-retro-4xl max-w-full overflow-hidden box-border leading-tight"
						>
							<span
								class="text-retro-purple min-w-[180px] max-w-[180px] font-medium flex-shrink-0 overflow-hidden text-ellipsis after:content-[' ']"
								>cpu_info_data:</span
							>
							<span class="text-retro-text flex-1 min-w-0 overflow-hidden break-words break-all">
								{arrayData.text}
								{#if arrayData.hasMore}
									<span
										class="text-retro-link cursor-pointer underline text-retro-sm font-mono transition-all duration-200 ease-in-out p-retro-xs bg-transparent hover:text-retro-text hover:bg-retro-highlight"
										on:click={() => (cpuDataExpanded = true)}
										on:keydown={(e) => e.key === 'Enter' && (cpuDataExpanded = true)}
										role="button"
										tabindex="0"
									>
										more]
									</span>
								{:else if cpuDataExpanded && arrayData.fullLength > 10}
									<span
										class="text-retro-link cursor-pointer underline text-retro-sm font-mono transition-all duration-200 ease-in-out p-retro-xs bg-transparent hover:text-retro-text hover:bg-retro-highlight"
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
		<div class="m-0 rounded-sm">
			<button
				class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 ease-in-out hover:text-retro-purple"
				on:click={() => (debugExpanded = !debugExpanded)}
			>
				<span class="text-retro-link mr-retro-md font-bold underline min-w-[12px] text-center"
					>{debugExpanded ? '-' : '+'}</span
				>
				<span class="text-retro-text font-medium">debug</span>
			</button>

			{#if debugExpanded}
				<div
					class="m-0 ml-retro-4xl p-retro-md pl-retro-xl bg-gray-50 border-2 border-dashed border-retro-border/30 overflow-hidden break-words max-w-[calc(100%-24px)] box-border rounded-retro-sm"
				>
					<pre
						class="text-retro-xs m-0 text-retro-muted whitespace-pre-wrap break-all break-words max-w-full overflow-hidden box-border">{systemInfo.debug}</pre>
				</div>
			{/if}
		</div>
	{/if}
</div>
