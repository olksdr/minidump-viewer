<script lang="ts">
	interface VersionInfo {
		file_version?: string;
		product_version?: string;
		file_flags?: string[];
		file_type?: string;
		file_os?: string;
	}

	interface CodeViewInfo {
		format: string;
		identifier?: string;
		age?: number;
		pdb_filename?: string;
	}

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

	interface ModuleData {
		modules: ModuleInfo[];
		modules_count: number;
		debug?: string;
	}

	export let moduleData: ModuleData;

	// State for collapsible sections - using module name as key
	let expandedModules = new Set<string>();
	let expandedSections: Record<string, boolean> = {}; // "moduleId-section" format

	// Helper to format values
	function formatValue(value: string | number | undefined | null): string {
		if (value === undefined || value === null) return '-';
		return String(value);
	}

	// Helper to format size in human readable format
	function formatSize(bytes: number): string {
		if (bytes === 0) return '0 bytes';
		const k = 1024;
		const sizes = ['bytes', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
	}

	// Helper to format timestamp
	function formatTimestamp(timestamp: number): string {
		if (timestamp === 0) return '-';
		try {
			const date = new Date(timestamp * 1000);
			return date.toISOString().replace('T', ' ').substring(0, 19) + ' UTC';
		} catch {
			return `0x${timestamp.toString(16).toLowerCase()}`;
		}
	}

	// Helper to format checksum
	function formatChecksum(checksum: number): string {
		if (checksum === 0) return '-';
		return `0x${checksum.toString(16).toLowerCase().padStart(8, '0')}`;
	}

	// Helper to toggle module expansion
	function toggleModule(moduleId: string) {
		if (expandedModules.has(moduleId)) {
			expandedModules.delete(moduleId);
		} else {
			expandedModules.add(moduleId);
		}
		expandedModules = new Set(expandedModules); // Trigger reactivity
	}

	// Helper to toggle nested sections
	function toggleSection(moduleId: string, section: string) {
		const key = `${moduleId}-${section}`;
		const oldValue = expandedSections[key] || false;
		expandedSections[key] = !oldValue;
		expandedSections = { ...expandedSections }; // Trigger reactivity
	}

	// Helper to generate module identifier for state tracking
	function getModuleId(module: ModuleInfo, index: number): string {
		// Use base address as primary identifier, fall back to name + index
		return module.base_of_image || `${module.name}-${index}`;
	}

	// Helper to get module filename from full path
	function getModuleFilename(fullPath: string): string {
		const parts = fullPath.split(/[\\/]/);
		return parts[parts.length - 1] || fullPath;
	}
</script>

<div class="font-mono text-retro-sm leading-tight whitespace-normal">
	{#each moduleData.modules as module, index (getModuleId(module, index))}
		{@const moduleId = getModuleId(module, index)}
		<div class="m-0 rounded-retro-sm">
			<button
				class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer py-retro-sm pr-0 pl-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-purple"
				on:click={() => toggleModule(moduleId)}
			>
				<span class="text-retro-purple mr-retro-md font-bold underline min-w-retro-3xl text-center"
					>{expandedModules.has(moduleId) ? '-' : '+'}</span
				>
				<span class="font-semibold mr-retro-xl text-retro-purple">
					{getModuleFilename(module.name)}
				</span>
				<span class="text-retro-muted text-retro-xs font-normal ml-retro-xl">
					({module.base_of_image}, {formatSize(module.size_of_image)})
				</span>
			</button>

			{#if expandedModules.has(moduleId)}
				<div
					class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-purple border-opacity-15"
				>
					<!-- Basic module info -->
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-45 font-medium after:content-[' ']">name:</span>
						<span class="text-retro-text flex-1 break-all">{module.name}</span>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
							>base_of_image:</span
						>
						<span class="text-retro-text flex-1 break-all">{module.base_of_image}</span>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
							>size_of_image:</span
						>
						<span class="text-retro-text flex-1 break-all"
							>{formatSize(module.size_of_image)} ({formatValue(module.size_of_image)} bytes)</span
						>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-45 font-medium after:content-[' ']">checksum:</span
						>
						<span class="text-retro-text flex-1 break-all">{formatChecksum(module.checksum)}</span>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
							>time_date_stamp:</span
						>
						<span class="text-retro-text flex-1 break-all"
							>{formatTimestamp(module.time_date_stamp)}</span
						>
					</div>
					<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
						<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
							>misc_record_present:</span
						>
						<span class="text-retro-text flex-1 break-all">{module.misc_record_present}</span>
					</div>

					<!-- Version Information -->
					{#if module.version_info}
						<div class="m-0 rounded-retro-sm">
							<button
								class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-purple"
								on:click|preventDefault|stopPropagation={() => toggleSection(moduleId, 'version')}
							>
								<span
									class="text-retro-purple mr-retro-md font-bold underline min-w-retro-3xl text-center"
									>{expandedSections[`${moduleId}-version`] ? '-' : '+'}</span
								>
								<span class="text-retro-text font-medium">version_info</span>
							</button>

							{#if expandedSections[`${moduleId}-version`]}
								<div
									class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-purple border-opacity-15"
								>
									{#if module.version_info.file_version}
										<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
											<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
												>file_version:</span
											>
											<span class="text-retro-text flex-1 break-all"
												>{module.version_info.file_version}</span
											>
										</div>
									{/if}
									{#if module.version_info.product_version}
										<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
											<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
												>product_version:</span
											>
											<span class="text-retro-text flex-1 break-all"
												>{module.version_info.product_version}</span
											>
										</div>
									{/if}
									{#if module.version_info.file_type}
										<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
											<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
												>file_type:</span
											>
											<span class="text-retro-text flex-1 break-all"
												>{module.version_info.file_type}</span
											>
										</div>
									{/if}
									{#if module.version_info.file_os}
										<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
											<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
												>file_os:</span
											>
											<span class="text-retro-text flex-1 break-all"
												>{module.version_info.file_os}</span
											>
										</div>
									{/if}
									{#if module.version_info.file_flags && module.version_info.file_flags.length > 0}
										<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
											<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
												>file_flags:</span
											>
											<span class="text-retro-text flex-1 break-all"
												>{module.version_info.file_flags.join(', ')}</span
											>
										</div>
									{/if}
								</div>
							{/if}
						</div>
					{/if}

					<!-- Debug Information -->
					{#if module.cv_record_info}
						<div class="m-0 rounded-retro-sm">
							<button
								class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-purple"
								on:click|preventDefault|stopPropagation={() => toggleSection(moduleId, 'debug')}
							>
								<span
									class="text-retro-purple mr-retro-md font-bold underline min-w-retro-3xl text-center"
									>{expandedSections[`${moduleId}-debug`] ? '-' : '+'}</span
								>
								<span class="text-retro-text font-medium">debug_info</span>
								<span class="text-retro-muted text-retro-xs font-normal ml-retro-xl">
									({module.cv_record_info.format})
								</span>
							</button>

							{#if expandedSections[`${moduleId}-debug`]}
								<div
									class="m-0 ml-retro-4xl p-0 pl-retro-xl bg-transparent border-l-2 border-retro-purple border-opacity-15"
								>
									<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
										<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
											>format:</span
										>
										<span class="text-retro-text flex-1 break-all"
											>{module.cv_record_info.format}</span
										>
									</div>
									{#if module.cv_record_info.identifier}
										<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
											<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
												>identifier:</span
											>
											<span class="text-retro-text flex-1 break-all"
												>{module.cv_record_info.identifier}</span
											>
										</div>
									{/if}
									{#if module.cv_record_info.age !== undefined && module.cv_record_info.age !== null}
										<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
											<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
												>age:</span
											>
											<span class="text-retro-text flex-1 break-all"
												>{module.cv_record_info.age}</span
											>
										</div>
									{/if}
									{#if module.cv_record_info.pdb_filename}
										<div class="flex m-0 p-0 pl-retro-4xl leading-tight">
											<span class="text-retro-purple min-w-45 font-medium after:content-[' ']"
												>pdb_filename:</span
											>
											<span class="text-retro-text flex-1 break-all"
												>{module.cv_record_info.pdb_filename}</span
											>
										</div>
									{/if}
								</div>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{/each}

	<!-- Debug Data -->
	{#if moduleData.debug}
		<div class="m-0 rounded-retro-sm">
			<button
				class="bg-transparent border-0 font-mono text-retro-sm text-retro-text cursor-pointer p-0 flex items-center w-full text-left transition-all duration-200 hover:text-retro-purple"
				on:click={() => toggleSection('global', 'debug')}
			>
				<span class="text-retro-purple mr-retro-md font-bold underline min-w-retro-3xl text-center"
					>{expandedSections['global-debug'] ? '-' : '+'}</span
				>
				<span class="text-retro-text font-medium">debug</span>
			</button>

			{#if expandedSections['global-debug']}
				<div
					class="m-0 ml-retro-4xl p-retro-md pl-retro-xl bg-gray-50 border-2 border-dashed border-retro-border/30 rounded-retro-sm"
				>
					<pre
						class="text-retro-xs m-0 text-retro-muted whitespace-pre-wrap break-words">{moduleData.debug}</pre>
				</div>
			{/if}
		</div>
	{/if}
</div>
